use std::{path::PathBuf, sync::{Arc, Mutex}};

use anyhow::{Error, Result};
use clap::{ArgAction, Parser, ValueEnum};
use colorize::AnsiColor;
use serde::Serialize;
use source_wand_dependency_analysis::{
    dependency_tree_node::DependencyTreeNode,
    dependency_tree_request::DependencyTreeRequest,
    find_build_requirements,
    find_dependency_tree,
    unique_dependencies_list::UniqueDependenciesList
};

#[derive(Debug, Parser)]
pub struct DependenciesArgs {
    path: String,

    #[arg(long, short)]
    checkout: Option<String>,

    #[arg(long, short, value_enum, default_value = "tree")]
    format: OutputFormat,

    #[arg(long, action = ArgAction::SetTrue)]
    flatten: bool,

    #[arg(long, action = ArgAction::SetTrue)]
    minimal_build_requirements: bool,

    #[arg(long, short)]
    export: Option<PathBuf>,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum OutputFormat {
    Tree,
    Json,
    Yaml,
}

#[derive(Debug, Clone)]
enum OutputData {
    Tree(Arc<Mutex<DependencyTreeNode>>),
    List(UniqueDependenciesList),
}

impl OutputData {
    pub fn to_string(&self) -> Result<String, String> {
        match self {
            OutputData::Tree(tree) => tree.lock().unwrap().to_string(),
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

fn is_git_url(s: &str) -> bool {
    s.starts_with("http://") ||
    s.starts_with("https://") ||
    s.starts_with("git://") ||
    s.starts_with("ssh://") ||
    s.starts_with("git@")
}

pub fn dependencies_command(args: &DependenciesArgs) -> Result<()> {
    let request: DependencyTreeRequest = if is_git_url(&args.path) {
        DependencyTreeRequest::GitProject {
            url: args.path.clone(),
            branch: args.checkout.clone(),
        }
    } else {
        if args.checkout.is_some() {
            eprintln!("{}", "Warning: The '--checkout' argument is only used for remote git repositories, it will be ignored.".yellow());
        }
        DependencyTreeRequest::LocalProject {
            path: PathBuf::from(&args.path),
        }
    };

    let dependency_tree = find_dependency_tree(request.clone())
        .map_err(|e| Error::msg(e))?;

    let output_data: OutputData = if args.minimal_build_requirements {
        let build_requirements = find_build_requirements(
            request,
            dependency_tree.clone(),
        ).map_err(|e| Error::msg(e))?;
        OutputData::List(build_requirements)
    }
    else if args.flatten {
        OutputData::List(dependency_tree.lock().unwrap().flatten())
    }
    else {
        OutputData::Tree(dependency_tree)
    };

    match args.format {
        OutputFormat::Tree => println!("{}", output_data.to_string().map_err(|e| Error::msg(e))?),
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&output_data)?),
        OutputFormat::Yaml => println!("{}", serde_yaml::to_string(&output_data)?),
    }

    Ok(())
}
