use std::collections::{HashMap, HashSet};
use std::time::Duration;
use std::thread::sleep;

use anyhow::{Error, Result};
use reqwest::blocking::{Client};
use reqwest::StatusCode;
use scraper::{Html, Selector};
use source_wand_common::{project::Project, project_manipulator::project_manipulator::ProjectManipulator};

use crate::dependency_tree_node::DependencyTreeNode;

pub fn generate_go_dependency_tree(
    project_manipulator: &dyn ProjectManipulator,
) -> Result<DependencyTreeNode> {
    let graph_raw: String = project_manipulator.run_shell("go mod graph".to_string())?;

    let mut dependencies_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut all_modules: HashSet<String> = HashSet::new();
    let mut child_modules: HashSet<String> = HashSet::new();

    for line in graph_raw.lines() {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        let is_valid_entry: bool = parts.len() == 2;

        if is_valid_entry {
            let parent: String = parts[0].to_string();
            let child: String = parts[1].to_string();

            dependencies_map.entry(parent.clone()).or_default().push(child.clone());

            all_modules.insert(parent.clone());
            all_modules.insert(child.clone());

            child_modules.insert(child);
        }
    }

    let roots: Vec<_> = all_modules.difference(&child_modules).cloned().collect();
    if roots.is_empty() {
        return Err(Error::msg("Could not determine root module"));
    }

    let root: &String = &roots[0];

    let mut project_cache: HashMap<String, Project> = HashMap::new();
    let mut repository_cache: HashMap<String, String> = HashMap::new();
    for module in &all_modules {
        let (name, version) = parse_module(module);
        let repository_url: String = extract_repository_url(&name, &mut repository_cache);
        project_cache.insert(
            module.clone(),
            Project::new(
                name,
                version,
                "".to_string(),
                repository_url
            ),
        );
    }

    let mut visited: HashSet<String> = HashSet::new();
    let tree: Box<DependencyTreeNode> = build_tree(root, &dependencies_map, &project_cache, &mut visited);

    Ok(*tree)
}

fn build_tree(
    root: &str,
    dependencies_map: &HashMap<String, Vec<String>>,
    project_cache: &HashMap<String, Project>,
    visited: &mut HashSet<String>,
) -> Box<DependencyTreeNode> {
    if visited.contains(root) {
        return Box::new(DependencyTreeNode::new(project_cache[root].clone(), vec![]));
    }
    visited.insert(root.to_string());

    let dependencies = dependencies_map
        .get(root)
        .unwrap_or(&vec![])
        .iter()
        .map(|dep| build_tree(dep, dependencies_map, project_cache, visited))
        .collect();

    Box::new(DependencyTreeNode::new(project_cache[root].clone(), dependencies))
}

fn parse_module(s: &str) -> (String, String) {
    if let Some((name, version)) = s.rsplit_once('@') {
        (name.to_string(), version.to_string())
    } else {
        (s.to_string(), "".to_string())
    }
}

fn extract_repository_url(module_path: &str, cache: &mut HashMap<String, String>) -> String {
    if let Some(url) = cache.get(module_path) {
        return url.clone();
    }

    let parts: Vec<&str> = module_path.split('/').collect();
    let has_at_least_three_parts: bool = parts.len() >= 3;

    let is_github: bool = has_at_least_three_parts && parts[0] == "github.com";
    let is_gitlab: bool = has_at_least_three_parts && parts[0] == "gitlab.com";
    let is_bitbucket: bool = has_at_least_three_parts && parts[0] == "bitbucket.org";
    let is_golang: bool = module_path.starts_with("golang.org/x/");

    let repository_url: String = if is_github || is_gitlab || is_bitbucket {
        format!("https://{}/{}/{}", parts[0], parts[1], parts[2])
    } else if is_golang {
        format!("https://go.googlesource.com/{}", &module_path["golang.org/x/".len()..])
    } else {
        match resolve_vanity_import(module_path) {
            Some(url) => url,
            None => format!("https://{}", module_path),
        }
    };

    cache.insert(module_path.to_string(), repository_url.clone());
    repository_url
}

fn resolve_vanity_import(module_path: &str) -> Option<String> {
    let url: String = format!("https://{}?go-get=1", module_path);
    let client = Client::new();
    let max_retries = 3;
    let base_delay = Duration::from_secs(1);

    for attempt in 0..max_retries {
        match client.get(&url).send() {
            Ok(response) => {
                if response.status() == StatusCode::TOO_MANY_REQUESTS {
                    let delay = base_delay * 2u32.pow(attempt);
                    sleep(delay);
                    continue;
                }
                let text = response.text().ok()?;
                let document: Html = Html::parse_document(&text);
                let selector: Selector = Selector::parse(r#"meta[name="go-import"]"#).ok()?;

                for element in document.select(&selector) {
                    if let Some(content) = element.value().attr("content") {
                        let parts: Vec<&str> = content.split_whitespace().collect();

                        if parts.len() == 3 {
                            return Some(parts[2].to_string());
                        }
                    }
                }
            }
            Err(e) => {
                if attempt == max_retries - 1 {
                    println!("Failed after {} retries: {}", max_retries, e);
                    return None;
                }
                // Calculate delay with exponential backoff
                let delay = base_delay * 2u32.pow(attempt);
                println!("Request error, retrying in {:?}: {}", delay, e);
                sleep(delay);
            }
        }
    }
    None
}
