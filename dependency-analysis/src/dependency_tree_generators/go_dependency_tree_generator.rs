use std::collections::{HashMap, HashSet};

use source_wand_common::{project::Project, project_manipulator::project_manipulator::ProjectManipulator};

use crate::dependency_tree_node::DependencyTreeNode;

pub fn generate_go_dependency_tree(
    project_manipulator: &dyn ProjectManipulator,
) -> Result<DependencyTreeNode, String> {
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
        return Err("Could not determine root module".to_string());
    }

    let root: &String = &roots[0];

    let mut project_cache: HashMap<String, Project> = HashMap::new();
    for module in &all_modules {
        let (name, version) = parse_module(module);
        let repository_url: String = extract_repository_url(&name);
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

fn extract_repository_url(module_path: &str) -> String {
    let parts: Vec<&str> = module_path.split('/').collect();

    let has_at_least_three_parts: bool = parts.len() >= 3;

    let is_github: bool = has_at_least_three_parts && parts[0] == "github.com";
    let is_gitlab: bool = has_at_least_three_parts && parts[0] == "gitlab.com";
    let is_bitbucket: bool = has_at_least_three_parts && parts[0] == "bitbucket.org";
    let is_golang: bool = module_path.starts_with("golang.org/x/");

    if is_github || is_gitlab || is_bitbucket {
        format!("https://{}/{}/{}", parts[0], parts[1], parts[2])
    }
    else if is_golang {
        format!("https://go.googlesource.com/{}", &module_path["golang.org/x/".len()..])
    } else {
        format!("https://{}", module_path)
    }
}
