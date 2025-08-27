use std::{env, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use source_wand_common::utils::write_yaml_file::write_yaml_file;
use source_wand_replication::model::{
    hooks::Hooks,
    package_destination_git::PackageDestinationGit,
    package_origin_git::PackageOriginGit,
    replication_manifest::ReplicationManifest
};

#[derive(Debug, Parser)]
pub struct InitArgs;

pub fn replicate_init_command(_args: &InitArgs) -> Result<()> {
    let working_directory: PathBuf = env::current_dir()?;
    let project_name = working_directory
        .file_name()
        .and_then(|os_string| os_string.to_str())
        .ok_or_else(|| anyhow::anyhow!("unknown"))?
        .to_string();

    let replication_manifest: ReplicationManifest = ReplicationManifest::new(
        project_name,
        Some(Hooks { before_all: None, before_each: None, after_each: None, after_all: None }),
        PackageOriginGit::new("<url to your project's repository>".to_string(), "<reference to checkout>".to_string()),
        PackageDestinationGit::new("<where to replicate your project>".to_string(), "<reference to push>".to_string()),
        None,
    );

<<<<<<< HEAD
    write_yaml_file(&replication_manifest, "replication.yaml")?;
=======
#[derive(Debug, Parser)]
pub struct GitDependenciesArgs {
    url: String,
    branch: Option<String>,
}

#[derive(Debug, Parser)]
pub struct NameDependenciesArgs {
    name: String,
    version: String,
}


pub fn _init_command(args: &InitArgs) -> Result<()> {
    println!("Planning project onboarding");

    println!(" > Generating dependency tree");
    // EDIT: Need to make this into a graph
    let dependency_tree: DependencyTreeNode = match &args.command {
        InitCommand::Local(args) => {
            find_dependency_tree(
                DependencyTreeRequest::LocalProject {
                    path: args.path.clone()
                }
            ).map_err(|e| Error::msg(e))?
        }
        InitCommand::Git(args) => {
            find_dependency_tree(
                DependencyTreeRequest::GitProject {
                    url: args.url.clone(),
                    branch: args.branch.clone(),
                }
            ).map_err(|e| Error::msg(e))?
        }
        InitCommand::ByName(args) => {
            find_dependency_tree(
                DependencyTreeRequest::NameBased {
                    name: args.name.clone(),
                    version: args.version.clone(),
                }
            ).map_err(|e|Error::msg(e))?
        }
    };

    let nb_dependencies: usize = dependency_tree.flatten().dependencies.len();
    println!(" > {} unique dependencies were found in the project", nb_dependencies);
    println!(" > Saving dependency tree");
    write_yaml_file(&dependency_tree, "dependencies.yaml")?;

    println!(" > Generating build requirements");

    let build_requirements: UniqueDependenciesList = match &args.command {
        InitCommand::Local(args) => {
            find_build_requirements(
                DependencyTreeRequest::LocalProject {
                    path: args.path.clone()
                },
                &dependency_tree,
            ).map_err(|e| Error::msg(e))?
        }
        InitCommand::Git(args) => {
            find_build_requirements(
                DependencyTreeRequest::GitProject {
                    url: args.url.clone(),
                    branch: args.branch.clone(),
                },
                &dependency_tree,
            ).map_err(|e| Error::msg(e))?
        }
        InitCommand::ByName(args) => {
            find_build_requirements(
                DependencyTreeRequest::NameBased {
                    name: args.name.clone(),
                    version: args.version.clone(),
                },
                &dependency_tree,
            ).map_err(|e|Error::msg(e))?
        }
    };

    let nb_build_requirements: usize = build_requirements.dependencies.len();
    println!(" > {} build requirements were found in the project", nb_build_requirements);
    println!(" > Saving build requirements");
    write_yaml_file(&build_requirements, "build-requirements.yaml")?;

    let nb_manual_requests: usize = plan_onboarding()?;
    if nb_manual_requests == 0 {
        println!("\nWhat to do next?");
        println!(" 1. source-wand onboard");
    }
    else {
        println!("\n > {} of {} build requirements require manual attention", nb_manual_requests, nb_build_requirements);
        println!("What to do next?");
        println!(" 1. Edit all ./to-complete/<name>-<version>/source-wand.yaml files manually");
        println!(" 2. source-wand apply-manual");
    }
>>>>>>> 516fbe8 (Can create a Graph)

    println!("Replication project initialized.");
    Ok(())
}
