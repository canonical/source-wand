use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use source_wand_dependency_analysis::{dependency_tree_request::DependencyTreeRequest, find_dependency_tree};

#[derive(Debug, Parser)]
pub struct DependenciesArgs {
    #[command(subcommand)]
    command: DependenciesCommand,

    #[arg(long, value_enum, default_value = "tree")]
    format: OutputFormat,
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

    match args.format {
        OutputFormat::Tree => println!("{}", dependency_tree.to_string()?),
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&dependency_tree).map_err(|e| e.to_string())?),
        OutputFormat::Yaml => println!("{}", serde_yaml::to_string(&dependency_tree).map_err(|e| e.to_string())?),
    }

    Ok(())
}
