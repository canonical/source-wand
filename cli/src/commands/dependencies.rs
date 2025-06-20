use std::path::PathBuf;

use anyhow::{Error, Result};
use clap::{ArgAction, Parser, Subcommand, ValueEnum};
use serde::Serialize;
use source_wand_dependency_analysis::{
    dependency_tree_node::DependencyTreeNode,
    dependency_tree_request::DependencyTreeRequest,
    find_dependency_tree,
    unique_dependencies_list::UniqueDependenciesList
};

#[derive(Debug, Parser)]
pub struct DependenciesArgs {
    #[command(subcommand)]
    command: DependenciesCommand,

    #[arg(long, value_enum, default_value = "tree")]
    format: OutputFormat,

    #[arg(long, action = ArgAction::SetFalse)]
    flatten: bool,
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

#[derive(Debug, Clone, ValueEnum)]
pub enum OutputFormat {
    Tree,
    Json,
    Yaml,
}

#[derive(Debug, Clone)]
enum OutputData {
    Tree(DependencyTreeNode),
    List(UniqueDependenciesList),
}

impl OutputData {
    pub fn to_string(&self) -> Result<String, String> {
        match self {
            OutputData::Tree(tree) => tree.to_string(),
            OutputData::List(list) => Ok(list.to_string()),
        }
    }
}

impl Serialize for OutputData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        match self {
            OutputData::Tree(tree) => tree.serialize(serializer),
            OutputData::List(list) => list.serialize(serializer),
        }
    }
}

pub fn dependencies_command(args: &DependenciesArgs) -> Result<()> {
    let dependency_tree = match &args.command {
        DependenciesCommand::Local(args) => {
            find_dependency_tree(
                DependencyTreeRequest::LocalProject {
                    path: args.path.clone()
                }
            ).map_err(|e| Error::msg(e))?
        },
        DependenciesCommand::Git(args) => {
            find_dependency_tree(
                DependencyTreeRequest::GitProject {
                    url: args.url.clone(),
                    branch: args.branch.clone(),
                }
            ).map_err(|e| Error::msg(e))?
        },
        DependenciesCommand::ByName(args) => {
            find_dependency_tree(
                DependencyTreeRequest::NameBased {
                    name: args.name.clone(),
                    version: args.version.clone(),
                }
            ).map_err(|e| Error::msg(e))?
        },
    };

    let output_data = if args.flatten {
        OutputData::Tree(dependency_tree)
    }
    else {
        OutputData::List(dependency_tree.flatten())
    };

    match args.format {
        OutputFormat::Tree => println!("{}", output_data.to_string().map_err(|e| Error::msg(e))?),
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&output_data)?),
        OutputFormat::Yaml => println!("{}", serde_yaml::to_string(&output_data)?),
    }

    Ok(())
}
