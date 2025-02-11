use std::path::PathBuf;

use clap::{Parser, Subcommand};
use source_wand_dependency_analysis::{dependency_tree_request::DependencyTreeRequest, find_dependency_tree};

#[derive(Debug, Parser)]
pub struct DependenciesArgs {
    #[command(subcommand)]
    command: DependenciesCommand
}

#[derive(Debug, Subcommand)]
pub enum DependenciesCommand {
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

pub fn dependencies_command(args: &DependenciesArgs) -> Result<(), String> {
    let dependency_tree = match &args.command {
        DependenciesCommand::Local(args) => {
            find_dependency_tree(
                DependencyTreeRequest::LocalProject {
                    path: args.path.clone()
                }
            )?
        },
        DependenciesCommand::Git(args) => {
            find_dependency_tree(
                DependencyTreeRequest::GitProject {
                    url: args.url.clone(),
                    branch: args.branch.clone(),
                }
            )?
        },
        DependenciesCommand::ByName(args) => {
            find_dependency_tree(
                DependencyTreeRequest::NameBased {
                    name: args.name.clone(),
                    version: args.version.clone(),
                }
            )?
        },
    };

    println!("{}", dependency_tree.to_string()?);

    Ok(())
}
