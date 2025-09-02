use std::{collections::HashSet, fs, path::PathBuf, str::FromStr, sync::Arc};
use anyhow::Result;
use reqwest::blocking::{get, Response};
use scraper::{Html, Selector};
use source_wand_common::{project::Project, project_manipulator::{
        local_project_manipulator::LocalProjectManipulator, project_manipulator::ProjectManipulator
    }};
use serde::{Deserialize, Serialize, Deserializer};
use uuid::Uuid;
use crate::dependency_tree_generators::go_depenendency_tree_struct::{DependencyTreeNodeGo, Graph};
use rayon::prelude::*; // 1. Import Rayon's parallel iterator traits


pub fn parse_dependency<'a>(
    url: &'a String,
    version: &'a String,
    project_root: &'a PathBuf,
    module_name: &'a String,
    graph: Arc<Graph<DependencyTreeNodeGo>>,
) {
    ///////////////////////// PRINT ///////////////////////////
    println!("###############NEW-CALL####################");
    println!("Module Name: {}", &module_name);
    println!("URL: {}", &url);
    println!("Version: {}", &version);
    println!("-------------------------------------------");
    ///////////////////////// PRINT ///////////////////////////
    //########## TODO #################
    // TODO: Check if the package is in sourcecraft. If it is, just create the node
    // 1. Module Name -> Check the Database (See if there is a sourcecraft name)
    // 2. Sourcecraft Name (+ version)) -> API (See if there is a track at the version)
    //########## TODO #################

    match graph.nodes.entry(module_name.clone()) {
        dashmap::mapref::entry::Entry::Occupied(_) => {
            return;
        }
        dashmap::mapref::entry::Entry::Vacant(entry) => {
            let path: PathBuf = PathBuf::from(format!(
                "{}/{}",
                project_root.to_string_lossy(),
                Uuid::new_v4().to_string()
            ));
            let mut checkout: Option<String> = None;
            let mut subdirectory:Option<String> = None;
            match fetch_checkout(&module_name, &version, &url) {
                Ok((checkout_vers, path)) => {
                    match checkout_vers {
                        Some(data) => checkout = Some(data),
                        None => checkout = None
                    }
                    match path {
                        Some(data2) => subdirectory = Some(data2),
                        None => subdirectory = None
                    }
                } Err(_e) => {

                }
            }
            let license: String = find_license(&module_name).unwrap_or("".to_string());
            
            let project_manipulator: LocalProjectManipulator = clone_repo(url, &checkout, &path);
            let _ = project_manipulator.run_shell(format!("go mod init {}", &module_name));
            let _ = project_manipulator.run_shell("sed -i 's/^go 1\\..*/go 1.18.0/' go.mod".to_string());
            let _ = project_manipulator.run_shell("go mod tidy".to_string());
            let _go_mod: String = match project_manipulator.run_shell("go mod edit -json".to_string()) {
                Ok(str) => str,
                Err(e) => { //TODO: Deal with error better in future
                    println!("{}", e.to_string());
                    return
                }, 
            }; 
            //println!("Go.Mod String: {}", &_go_mod);
            let _go_mod_parsed: Option<GoMod> = match serde_json::from_str(&_go_mod) {
                Ok(gm) => gm,
                Err(e) => {
                    println!("ERROR!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
                    println!("{}", &module_name);
                    eprintln!("Failed to deserialize: {}", e);
                    None
                }
            };
            // STEP: Create A New Project //

            let new_project: Project = Project::new(module_name.clone(), version.clone(), license, url.clone(), subdirectory.clone(), checkout.clone());
            let new_node: DependencyTreeNodeGo = DependencyTreeNodeGo::new(new_project);
            entry.insert(new_node);
            println!("@@@@@ Added New Node: {}", module_name);

            if let Some(go_mod_parsed) = _go_mod_parsed {
                let parent: String = go_mod_parsed.module.path.clone();
                if let Some(requires) = go_mod_parsed.require {
                    let children_to_process: Vec<(String, String, String)> = requires.par_iter().filter_map(|dep| {
                        let child = dep.path.clone();
                        if !graph.does_key_exist(&child) {  // Rough check; entry in recursion will handle races
                            Some((dep.path.clone(), dep.version.clone(), get_repository_url(&dep.path)))
                        } else {
                            None
                        }
                    }).collect();
                    children_to_process.par_iter().for_each(|(child_path, child_version, child_url)| {
                        let graph_clone = graph.clone();
                        parse_dependency(child_url, child_version, project_root, child_path, graph_clone);
                    });
                    requires.par_iter().for_each(|dep| {
                        let child = dep.path.clone();
                        println!("## Parent: {} | Child: {}", &parent, &child);
                        graph.edges.entry(parent.clone())
                            .or_insert_with(HashSet::new)
                            .insert(child.clone());
                        println!("@@@@ dependency {} has dep {}", &parent, &child);
                    });
                }
            }
            project_manipulator.cleanup();
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////
// ################### HELPER FUNCTIONS #######################


/// Resolves the Go module name to the repository URL (for `git clone`)
/// 
/// # Example:
/// let go_module_name: &str = "github.com/davecgh/go-spew"
/// get_repository_url(go_module_name) => https://github.com/davecgh/go-spew
/// 
//TODO: Bring back the cache from `extract_repository_url` in `go_dependency_tree_generator.rs`
fn get_repository_url(module_name: &str) -> String {
    let parts: Vec<&str> = module_name.split('/').collect();
    let has_at_least_three_parts: bool = parts.len() >= 3;

    let is_github: bool = has_at_least_three_parts && parts[0] == "github.com";
    let is_gitlab: bool = has_at_least_three_parts && parts[0] == "gitlab.com";
    let is_bitbucket: bool = has_at_least_three_parts && parts[0] == "bitbucket.org";
    let is_golang: bool = module_name.starts_with("golang.org/x/");

    let repository_url: String = if is_github || is_gitlab || is_bitbucket {
        format!("https://{}/{}/{}", parts[0], parts[1], parts[2])
    } else if is_golang {
        format!("https://go.googlesource.com/{}", &module_name["golang.org/x/".len()..])
    } else {
        match get_repository_url_pkg_go_dev(module_name) {
            Some(url) => url,
            None => format!("https://{}", module_name),
        }
    };
    repository_url
}

/// Resolve Go module name to the repository URL (using pkg.go.dev)
fn get_repository_url_pkg_go_dev(module_path: &str) -> Option<String> {
    let url: String = format!("https://{}?go-get=1", module_path);

    let document: Html = Html::parse_document(&get(&url).ok()?.text().ok()?);
    let selector: Selector = Selector::parse(r#"meta[name="go-import"]"#).ok()?;

    for element in document.select(&selector) {
        if let Some(content) = element.value().attr("content") {
            let parts: Vec<&str> = content.split_whitespace().collect();

            if parts.len() == 3 {
                return Some(parts[2].to_string());
            }
        }
    }
    None
}

/// Clone the repository at the URL, Version, and Project Root (The path to clone to)
/// Return the manipulator that's at the root of the repository
fn clone_repo(url: &String, checkout: &Option<String>, project_root: &PathBuf) -> LocalProjectManipulator {
    let mut repo_root: PathBuf = project_root.clone();
    repo_root.push(Uuid::new_v4().to_string());
    let result = fs::create_dir_all(&repo_root);
    if let Err(e) = result {
        eprintln!("Failed to create directory: {}", e);
    } else {
        println!("Successfully created directory!");
    }
    let manipulator: LocalProjectManipulator = LocalProjectManipulator::new(
        repo_root, true);
    match manipulator.try_run_shell(format!("git clone \"{}\" .", url,), 20) {
        Ok(str) => println!("Clone: {}", str),
        Err(e) => eprintln!("Error: {}", e),
    }
    // Get the correct checkout 
    if let Some(ver) = &checkout {
        match manipulator.run_shell(format!("git checkout {}", ver)) {
            Ok(str) => println!("Checkout: {}", str),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    return manipulator;
}

/// Resolves the version & checkout from the "version" given by Go
/// Version: The version we want to write on our sourcecraft.yaml
/// Checkout: The checkout version we want on our build
/// (Checkout, Subdirectory)
fn fetch_checkout(name: &String, version: &String, repository: &String) -> Result<(Option<String>, Option<String>)> {
    // Case 1: v1.2.3 >> (v1.2.3)
    let project_manipulator: LocalProjectManipulator = LocalProjectManipulator::new(PathBuf::from_str("/")?, true);
    let short_name: &str = match name.split("/").last() {
        Some(short_name) => short_name,
        None => name.as_str(),
    };

    let tags_raw: String = project_manipulator.run_shell(format!("git ls-remote --tags {}", repository))?;
    let tags: Vec<&str> = tags_raw.lines()
        .into_iter()
        .filter_map(|tag| tag.split("\t").last())
        .collect();

    let branches_raw: String = project_manipulator.run_shell(format!("git ls-remote --heads {}", repository))?;
    let branches: Vec<&str> = branches_raw.lines()
        .filter_map(|branch| branch.split("\t").last())
        .collect();

    let mut path: Option<String> = None;

    let version_tag: &str = version.split('+').next().unwrap_or(&version);
    let checkout: Option<String> =
        //For Monorepositories
        if let Some(tag) = tags.iter().find(|tag| tag.contains(&format!("{}/{}", short_name, version_tag))) {
            path = Some(short_name.to_string());
            Some(tag.to_string())
        }
        else if let Some(tag) = tags.iter().find(|tag| tag.contains(version_tag)) {
            Some(tag.to_string())
        }
        else if let Some(branch) = branches.iter().find(|branch| branch.contains(version_tag)) {
            Some(branch.to_string())
        }
        else {
            None
        };

    Ok((checkout, path))
}

fn find_license(module_path: &str) -> Option<String> {
    let url: String = format!("https://pkg.go.dev/{}?go-get=1", module_path);

    let response: Response = match get(&url) {
        Ok (resp) => resp,
        Err(e) => {
            eprintln!("Failed to fetch URL {}: {}", url, e);
            return None;
        }
    };
    let html_text: String = match response.text() {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Failed to get HTML text: {}", e);
            return None;
        }
    };

    let document: Html = Html::parse_document(&html_text);
    let selector: Selector = Selector::parse("span").expect("Failed to parse selector");

    for element in document.select(&selector) {
        let text = element.text().collect::<Vec<_>>().join("");
        if text.contains("License:") {
            let license = text.replace("License: ", "").trim().to_string();
            return Some(license);
        }
    }

    None
}


////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(dead_code)]
struct GoMod {
    pub module: Module,
    #[serde(rename = "Go")]
    pub go_version: Option<String>,
    pub require: Option<Vec<Require>>,
    pub exclude: Option<Vec<Exclude>>,
    pub replace: Option<Vec<Replace>>,
    pub retract: Option<Vec<Retract>>,
    pub tool: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Module {
    path: String,
}


// Represents an object in the "Require" array
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(dead_code)]
struct Require {
    path: String,
    version: String,
    indirect: Option<bool>,
}

// Represents an object in the "Exclude" array
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(dead_code)]
struct Exclude {
    path: String,
    version: String,
}

// Represents an object in the "Replace" array
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(dead_code)]
struct Replace {
    old: ModuleVersion,
    new: ModuleVersion,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(dead_code)]
struct Retract {
    low: String,
    high: String,
    rationale: Option<String>,
}

// Represents the "Old" and "New" objects within a "Replace" object
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(dead_code)]
struct ModuleVersion {
    path: String,
    version: Option<String>,
}