use std::path::PathBuf;

use anyhow::{Error, Result};
use clap::Parser;
use source_wand_common::utils::write_yaml_file::write_yaml_file;
use source_wand_dependency_analysis::{
    dependency_tree_node::DependencyTreeNode,
    dependency_tree_request::DependencyTreeRequest,
    find_build_requirements,
    find_dependency_tree,
    unique_dependencies_list::UniqueDependenciesList
};
use source_wand_onboarding::plan::plan_onboarding::plan_onboarding;

#[derive(Debug, Parser)]
pub struct InitArgs {
    #[command(subcommand)]
    command: InitCommand,
}

#[derive(Debug, Parser)]
pub enum InitCommand {
    #[command(about = "From a local project.")]
    Local(LocalDependenciesArgs),
    #[command(about = "From a project in a git repository.")]
    Git(GitDependenciesArgs),
    #[command(about = "From the name/version pair of a project.")]
    ByName(NameDependenciesArgs),
}

#[derive(Debug, Parser)]
pub struct LocalDependenciesArgs {
    path: PathBuf,
}

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


pub fn init_command(args: &InitArgs) -> Result<()> {
    println!("Planning project onboarding");

    println!(" > Generating dependency tree");
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

    Ok(())
}
