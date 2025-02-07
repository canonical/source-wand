use std::path::PathBuf;

use clap::{Parser, Subcommand};
use source_wand_dependency_analysis::{dependency_tree_request::DependencyTreeRequest, find_dependency_tree};

#[derive(Debug, Parser)]
pub struct DeptreeArgs {
    #[command(subcommand)]
    command: DeptreeCommand
}

#[derive(Debug, Subcommand)]
pub enum DeptreeCommand {
    #[command(about = "From a local project.")]
    Local(LocalDeptreeArgs),
    #[command(about = "From a project in a git repository.")]
    Git(GitDeptreeArgs),
    #[command(about = "From the name/version pair of a project.")]
    ByName(NameDeptreeArgs),
}

#[derive(Debug, Parser)]
pub struct LocalDeptreeArgs {
    path: PathBuf,
}

#[derive(Debug, Parser)]
pub struct GitDeptreeArgs {
    url: String,
    branch: Option<String>,
}

#[derive(Debug, Parser)]
pub struct NameDeptreeArgs {
    name: String,
    version: String,
}

pub fn deptree_command(args: &DeptreeArgs) -> Result<(), String> {
    let dependency_tree = match &args.command {
        DeptreeCommand::Local(args) => {
            find_dependency_tree(
                DependencyTreeRequest::LocalProject {
                    path: args.path.clone()
                }
            )?
        },
        DeptreeCommand::Git(args) => {
            find_dependency_tree(
                DependencyTreeRequest::GitProject {
                    url: args.url.clone(),
                    branch: args.branch.clone(),
                }
            )?
        },
        DeptreeCommand::ByName(args) => {
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
