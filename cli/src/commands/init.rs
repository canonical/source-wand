use std::path::PathBuf;

use anyhow::{Error, Result};
use clap::Parser;
use source_wand_common::utils::write_yaml_file::write_yaml_file;
use source_wand_dependency_analysis::{
    dependency_tree_request::DependencyTreeRequest,
    find_dependency_tree
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

    let dependency_tree = match &args.command {
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

    /*
    let dependency_tree: DependencyTreeNode = find_dependency_tree(
        DependencyTreeRequest::GitProject {
            url: args.from_git.clone(),
            branch: args.checkout.clone(),
        }
    ).map_err(|e| Error::msg(e))?;
    */
    let nb_dependencies: usize = dependency_tree.flatten().dependencies.len();
    println!(" > {} unique dependencies were found in the project", nb_dependencies);
    println!(" > Saving dependency tree");
    write_yaml_file(&dependency_tree, "dependencies.yaml")?;

    let nb_manual_requests: usize = plan_onboarding()?;
    if nb_manual_requests == 0 {
        println!("\nWhat to do next?");
        println!(" 1. source-wand onboard");
    }
    else {
        println!("\n > {} of {} dependencies require manual attention", nb_manual_requests, nb_dependencies);
        println!("What to do next?");
        println!(" 1. Edit all ./to-complete/<name>-<version>/source-wand.yaml files manually");
        println!(" 2. source-wand apply-manual");
    }

    Ok(())
}
