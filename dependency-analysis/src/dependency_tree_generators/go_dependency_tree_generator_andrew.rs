use std::{ fs, path::PathBuf, str::FromStr, sync::{Arc, Mutex}};
use futures::future::join_all;

use anyhow::Result;
use reqwest::blocking::get;
use scraper::{Html, Selector};
use source_wand_common::project_manipulator::{
        self, local_project_manipulator::{LocalProjectManipulator, LocalProjectManipulatorAsync, ProjectManipulatorAsync}, project_manipulator::{AnyProjectManipulator, ProjectManipulator}
    };
use serde::{Deserialize};
use uuid::Uuid;
use crate::{dependency_tree_generators::go_depenendency_tree_struct::{DependencyTreeNodeGo, GoProject, Graph} };
use rayon::prelude::*; // 1. Import Rayon's parallel iterator traits



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

/**
 * 
 * 
 * 
 */
pub async fn parse_dependency<'a>(
    url: String,
    version: String,
    project_root: PathBuf,
    module_name: String,
    graph: Arc<Mutex<Graph<DependencyTreeNodeGo, String>>>,
) -> Result<()> {
    ///////////////////////// PRINT ///////////////////////////
    println!("###############NEW-CALL####################");
    println!("Module Name: {}", &module_name);
    println!("URL: {}", &url);
    println!("Version: {}", &version);
    println!("###########################################");
    ///////////////////////// PRINT ///////////////////////////
    //########## TODO #################
    // TODO: Check if the package is in sourcecraft. If it is, just create the node
    // 1. Module Name -> Check the Database (See if there is a sourcecraft name)
    // 2. Sourcecraft Name (+ version)) -> API (See if there is a track at the version)

    //########## TODO #################
    if graph.lock().unwrap().does_key_exist(&module_name) {
        return Ok(());
    }
    // STEP: Clone the repository & parse the Go.Mod
    let path: PathBuf = PathBuf::from(format!(
        "{}/{}",
        project_root.to_string_lossy(),
        Uuid::new_v4().to_string()
    ));
    let (checkout, _subdirectory) = fetch_checkout(&module_name, &version, &url).await
        .unwrap_or((Some(version.clone()), None));
    let checkout = checkout.unwrap_or_default();

    //match fetch_checkout(&module_name, &version, &url) {
    //    Ok((checkout_vers, path)) => {
    //        match checkout_vers {
    //            Some(data) => checkout = data,
    //            None => checkout = String::from(""),
    //        }
    //        match path {
    //            Some(data2) => subdirectory = data2,
    //            None => subdirectory = String::from(""),
    //        }
    //    } Err(_e) => {
    //    }
    //}
    let project_manipulator: LocalProjectManipulatorAsync = clone_repo(&url, &checkout, &path).await?;
    let _ = project_manipulator.run_shell(format!("go mod init {}", &module_name)).await;
    project_manipulator.run_shell("sed -i 's/^go 1\\..*/go 1.18.0/' go.mod".to_string()).await?;
    project_manipulator.run_shell("go mod tidy".to_string()).await?;
    let _go_mod: String = project_manipulator.run_shell("go mod edit -json".to_string()).await?;
    //println!("Go.Mod String: {}", &_go_mod);
    println!("Go Mod JSON for {}: {}", &module_name, &_go_mod);
    let _go_mod_parsed: Option<GoMod> = match serde_json::from_str(&_go_mod) {
        Ok(gm) => gm,
        Err(e) => {
            println!("ERROR!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
            println!("{}", &module_name);
            eprintln!("Failed to deserialize: {}", e);
            None
        }
    };
    println!("Parsed Go Mod for {}: {:?}", &module_name, &_go_mod_parsed);
    // STEP: Create A New Project //

    let new_project: GoProject = GoProject::new(module_name.clone(), checkout.clone(), url.clone(), checkout.clone());
    let new_node: DependencyTreeNodeGo = DependencyTreeNodeGo::new(new_project);
    // Add key as Module-Version(Checkout)
    graph.lock().unwrap().add_node(module_name.clone(), new_node);
    println!("@@@@@ Added New Node: {}-{}", module_name, checkout);

    if let Some(parsed) = _go_mod_parsed {
        if let Some(requires) = parsed.require {
            let tasks: Vec<_> = requires.into_iter().map(|dep| {
                let parent: String = module_name.clone();
                let child: String = dep.path.clone();
                println!("&&&&&&&&&& Parent: {} | Child: {} &&&&&&&&&&&", &parent, &child);
                let dep_url: String = get_repository_url(&dep.path);
                let graph_clone = Arc::clone(&graph);
                let project_root = project_root.clone();

                async move {
                    graph_clone.lock().unwrap().add_depends(&parent, &child);
                    parse_dependency(dep_url, dep.version, project_root, dep.path, graph_clone).await
                }
            }).collect();
            let results = join_all(tasks).await;
            for result in results {
                if let Err(e) = result {
                    eprintln!("A sub-task failed: {}", e);
                }
            }
        }
    }

    println!("COMPLETED: {}", module_name);
    Ok(())
        //////////////////
        //go_mod_parsed.require.par_iter().flatten().for_each(|dep| {
        //    let parent: String = go_mod_parsed.module.path.clone();
        //    let child: String = dep.path.clone();
        //    println!("&&&&&&&&&& Parent: {} | Child: {} &&&&&&&&&&&", &parent, &child);
        //    let needs_recursion = !graph.lock().unwrap().does_key_exist(&child);

        //    if needs_recursion {
        //        println!("Key Doesn't Exist. Need to create {}", &child);
        //        let dep_url: String = get_repository_url(&dep.path);
        //        let graph_clone = Arc::clone(&graph);
        //        parse_dependency(&dep_url, &dep.version, &project_root, &dep.path, graph_clone);
        //    } else {
        //        println!("Key Exists: {}",&child);
        //    }
        //    graph.lock().unwrap().add_depends(&parent, &child);
        //    println!("@@@@@@ {} >>> {} @@@@@@@@@", &parent, &child);
        //});
        ////////////////////////////
    //}
    //project_manipulator.cleanup();
}

                //println!("@@@@@@ {} >>> {} @@@@@@@@@", &parent, &child);
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
async fn clone_repo(url: &String, version: &String, project_root: &PathBuf) -> Result<LocalProjectManipulatorAsync> {
    let mut repo_root: PathBuf = project_root.clone();
    repo_root.push(Uuid::new_v4().to_string());
    let result = fs::create_dir_all(&repo_root);
    if let Err(e) = result {
        eprintln!("Failed to create directory: {}", e);
    } else {
        println!("Successfully created directory!: {:#?}", &repo_root);
    }
    let manipulator: LocalProjectManipulatorAsync = LocalProjectManipulatorAsync::new(
        repo_root, true);
    println!("{:#?}", manipulator.get_working_directory());
    manipulator.try_run_shell(format!("git clone \"{}\" .", url), 2).await?;
    let v = get_version(&version); //TODO: Replace this function with `fetch_checkout`
    manipulator.run_shell(format!("git checkout {}", v)).await?;
    return Ok(manipulator);
}

/// Resolves the version & checkout from the "version" given by Go
/// Version: The version we want to write on our sourcecraft.yaml
/// Checkout: The checkout version we want on our build
/// (Checkout, Subdirectory)
async fn fetch_checkout(name: &String, version: &String, repository: &String) -> Result<(Option<String>, Option<String>)> {
    // Case 1: v1.2.3 >> (v1.2.3)
    let project_manipulator: LocalProjectManipulatorAsync = LocalProjectManipulatorAsync::new(PathBuf::from_str("/")?, true);
    let short_name: &str = match name.split("/").last() {
        Some(short_name) => short_name,
        None => name.as_str(),
    };

    let tags_raw: String = project_manipulator.run_shell(format!("git ls-remote --tags {}", repository)).await?;
    let tags: Vec<&str> = tags_raw.lines()
        .into_iter()
        .filter_map(|tag| tag.split("\t").last())
        .collect();

    let branches_raw: String = project_manipulator.run_shell(format!("git ls-remote --heads {}", repository)).await?;
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


fn get_version(version_string: &str) -> &str {
    if let Some((_, commit_hash)) = version_string.rsplit_once('-') {
        // This is a pre-release version, return the commit hash
        commit_hash
    } else {
        // This is a semantic version, return the original string
        version_string
    }
}






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