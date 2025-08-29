use std::{collections::{HashMap, HashSet}, env, fs, path::PathBuf, str::FromStr, string};

use anyhow::Result;
use reqwest::blocking::get;
use scraper::{Html, Selector};
use source_wand_common::project_manipulator::{
        self, local_project_manipulator::LocalProjectManipulator, project_manipulator::{AnyProjectManipulator, ProjectManipulator}
    };
use serde::{Deserialize, Serialize, Deserializer};
use serde_json::Value;
use uuid::Uuid;
use crate::{dependency_tree_generators::go_depenendency_tree_struct::{DependencyTreeNodeGo, GoProject, Graph}, dependency_tree_node::DependencyTreeNode};

// This is the function you need to implement.
pub fn deserialize_require<'de, D>(deserializer: D) -> Result<Vec<GoModNode>, D::Error>
where
    D: Deserializer<'de>,
{
    // Check if the incoming value is `null` or a `None`
    let value: Option<Vec<GoModNode>> = Option::deserialize(deserializer)?;
    
    // If the value is `Some`, unwrap it. Otherwise, return an empty vector.
    Ok(value.unwrap_or_else(|| Vec::new()))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoModFile {
    #[serde(rename = "Module")]
    pub module: GoModModule,
    #[serde(rename = "Go")]
    pub go_version: String,
    #[serde(rename = "Require")]
    #[serde(deserialize_with = "deserialize_require")]
    pub require: Vec<GoModNode>,
    #[serde(rename = "Exclude")]
    pub exclude: Option<String>,
    #[serde(rename = "Replace")]
    pub replace: Option<String>,
    #[serde(rename = "Retract")]
    pub retract: Option<String>,
    #[serde(rename = "Tool")]
    pub tool: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoModNode {
    #[serde(rename = "Path")]
    pub path: String,
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "Indirect")]
    pub indirect: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoModModule {
    #[serde(rename = "Path")]
    pub path: String
}

fn get_version(version_string: &str) -> &str {
    if let Some((_, commit_hash)) = version_string.rsplit_once('-') {
        // This is a pre-release version, return the commit hash
        commit_hash
    } else {
        // This is a semantic version, return the original string
        version_string
    }
}

/**
 * first dependency
 * 1. Create a graph (empty)
 * 2. Start with the URL, Version (For the top-level)
 * 3. Feed to the parse_dependency (starting) -> have it return the graph
 * 
 */
//pub fn generate_go_dependency_tree_andrew(url: &String, version: &String, project_root: &PathBuf) -> Result<Graph<DependencyTreeNodeGo, String>> {
//    let mut graph: Graph<DependencyTreeNodeGo, String> = Graph::new();
//    let project_manipulator: LocalProjectManipulator = clone_repo(url, version, project_root);
//    let _go_mod: String = match project_manipulator.run_shell("go mod edit -json".to_string()) {
//        Ok(str) => str,
//        Err(e) => e.to_string(), // Deal with error better in future
//    }; //TODO: #now Error handling
//    let _go_mod_parsed: Option<GoModFile> = match serde_json::from_str(&_go_mod) { //TODO: #now Error handling
//        Ok(gm) => gm,
//        Err(e) => {
//            eprintln!("Failed to deserialize: {}", e);
//            None
//        }
//    };
//    let checkout: String = version.clone();
//
//    let new_project: GoProject = GoProject::new("github.com/canonical/chisel".to_string(), url.clone(), checkout);
//    let new_node: DependencyTreeNodeGo = DependencyTreeNodeGo::new(new_project);
//    graph.add_node("github.com/canonical/chisel".to_string().clone(), new_node);
//    if _go_mod_parsed.is_some() {
//        let go_mod_parsed: GoModFile = _go_mod_parsed.unwrap();
//        for dep in &go_mod_parsed.require { // Change into a map
//            let parent: String = go_mod_parsed.module.path.clone(); // The key doesn't include version
//            let child: String = dep.path.clone();
//            if graph.does_key_exist(&parent) {
//                println!("Found the key");
//            } else {
//                // Create The Dependency
//                parse_dependency(&dep.path, &dep.version, &project_root, &dep.path, &mut graph);
//            }
//            graph.add_depends(&parent, &child);
//        }
//    }
//    Ok(graph)
//}

/**
 * 
 * 
 * 
 */
pub fn parse_dependency<'a>(
    url: &'a String,
    version: &'a String,
    project_root: &'a PathBuf,
    module_name: &'a String,
    graph: &'a mut Graph<DependencyTreeNodeGo, String>,
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
    // STEP: Clone the repository & parse the Go.Mod
    let path: PathBuf = PathBuf::from(format!(
        "{}/{}",
        project_root.to_string_lossy(),
        Uuid::new_v4().to_string()
    ));
    let project_manipulator: LocalProjectManipulator = clone_repo(url, version, &path);
    let _ = project_manipulator.run_shell("sed -i 's/^go 1\\..*/go 1.18.0/' go.mod".to_string());
    let _ = project_manipulator.run_shell(format!("go mod init {}", &module_name));
    let _ = project_manipulator.run_shell("go mod tidy".to_string());
    let _go_mod: String = match project_manipulator.run_shell("go mod edit -json".to_string()) {
        Ok(str) => str,
        Err(e) => { //TODO: Deal with error better in future
            e.to_string();
            return
        }, 
    }; 
    println!("Go.Mod String: {}", &_go_mod);
    let _go_mod_parsed: Option<GoModFile> = match serde_json::from_str(&_go_mod) {
        Ok(gm) => gm,
        Err(e) => {
            eprintln!("Failed to deserialize: {}", e);
            None
        }
    };
    // STEP: Create A New Project //
    // # Fields
    let mut checkout = String::new();
    let mut subdirectory = String::new();
    match fetch_checkout(&module_name, &version, &url) {
        Ok((checkout_vers, path)) => {
            match checkout_vers {
                Some(data) => checkout = data,
                None => checkout = String::from(""),
            }
            match path {
                Some(data2) => subdirectory = data2,
                None => subdirectory = String::from(""),
            }
        } Err(_e) => {

        }
    }

    let new_project: GoProject = GoProject::new(module_name.clone(), checkout.clone(), url.clone(), checkout.clone());
    let new_node: DependencyTreeNodeGo = DependencyTreeNodeGo::new(new_project);
    // Add key as Module-Version(Checkout)
    graph.add_node(module_name.clone(), new_node);
    println!("@@@@@ Added New Node: {}-{}", module_name, checkout);

    if _go_mod_parsed.is_some() {
        println!("^^Go Mod Parsed Exists (Doesn't Fail)");
        let go_mod_parsed: GoModFile = _go_mod_parsed.unwrap();
        for dep in &go_mod_parsed.require {
            let parent: String = go_mod_parsed.module.path.clone();
            let child: String = dep.path.clone();



            println!("## Parent: {} | Child: {}", &parent, &child);



            if graph.does_key_exist(&child) {
                println!("Found the key");
            } else {
                println!("Key Doesn't Exist. Need to create {}", &child);
                let dep_url: String = get_repository_url(&dep.path);
                parse_dependency(&dep_url, &dep.version, &project_root, &dep.path, graph);
            }
            graph.add_depends(&parent, &child);
            println!("@@@@ dependency {} has dep {}", &parent, &child);
        }
    }
    project_manipulator.cleanup();
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
fn clone_repo(url: &String, version: &String, project_root: &PathBuf) -> LocalProjectManipulator {
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
    let v = get_version(&version);
    match manipulator.run_shell(format!("git checkout {}", v)) {
        Ok(str) => println!("Checkout: {}", str),
        Err(e) => eprintln!("Error: {}", e),
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

//fn resolve_checkout(version: &String, repository: &String) -> Option<String> {
//    let path = PathBuf::from_str("/");
//    let project_manipulator: LocalProjectManipulator = LocalProjectManipulator::new(path, true);
//    let tags_raw: String = project_manipulator.run_shell(format!("git ls-remote --tags {}", repository))?;
//    let tags: Vec<&str> = tags_raw.lines()
//        .into_iter()
//        .filter_map(|tag| tag.split("\t").last())
//        .collect();
//
//    let branches_raw: String = project_manipulator.run_shell(format!("git ls-remote --heads {}", repository))?;
//    let branches: Vec<&str> = branches_raw.lines()
//        .filter_map(|branch| branch.split("\t").last())
//        .collect();
//    let version_tag: &str = version.split('+').next().unwrap_or(&version);
//
//    let checkout: Option<String> =
//        if let Some(tag) = tags.iter().find(|tag| tag.contains(version_tag)) {
//            Some(tag.to_string())
//        }
//        else if let Some(branch) = branches.iter().find(|branch| branch.contains(version_tag)) {
//            Some(branch.to_string())
//        }
//        else {
//            None
//        };
//    checkout
//}


    


    //let name: String = "".to_string();
    //let (version, checkout) = match resolve_version_and_checkout(&name, version, repository) {
    //    (Some(version), Some(checkout)) => (Some(version), Some(checkout)),
    //    (None, None) => (None, None),
    //    _ => (None, None),
    //};
    //let version: String = "".to_string();
    //let license: String = "".to_string();
    //let repository_url: String = "".to_string();
    //let subdirectory: String = "".to_string();
    //let checkout: String = "".to_string();

    //let new_project: GoProject = GoProject::new(name, version, license, repository_url, subdirectory, checkout);

    //let (checkout, subdirectory) : String =  match fetch_checkout(&module_name, &version, &url) {
    //    Ok((checkout, subdirectory)) => (checkout, subdirectory),
    //    Err(_) => (None, None)
    //};
    
    
    
    //let checkout: String = match resolve_checkout(&version, &url) {
    //    Some(str) => str,
    //    None => {
    //        eprintln!("For this repository {} and version {}, there is no match.", &version, &url);
    //        "".to_string()
    //    }
    //};