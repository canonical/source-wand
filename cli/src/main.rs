use std::{env, path::PathBuf, sync::Arc};

use anyhow::Result;
use clap::{Parser, Subcommand};
use commands::{
    dependencies::{
        dependencies_command,
        DependenciesArgs
    }
};
use source_wand_dependency_analysis::dependency_tree_generators::{go_dependency_tree_generator_andrew::parse_dependency, go_depenendency_tree_struct::{DependencyTreeNodeGo, Graph}};
use uuid::Uuid;

use crate::commands::{
    apply::{
        replicate_apply_command,
        ApplyArgs
    },
    init::{
        replicate_init_command,
        InitArgs
    },
    plan::{
        replicate_plan_command,
        PlanArgs
    },
};

mod commands;

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[command(about = "Find the dependency tree of a project.")]
    Dependencies(DependenciesArgs),

    #[command(about = "Initialize a new deep replication project")]
    Init(InitArgs),

    #[command(about = "Plan a deep replication and validate the replication is possible")]
    Plan(PlanArgs),

    #[command(about = "Apply the deep replication plan")]
    Apply(ApplyArgs),
}

//fn main() -> Result<()> {
//    match Cli::parse().command {
//        Command::Dependencies(args) => dependencies_command(&args),
//        Command::Init(args) => replicate_init_command(&args),
//        Command::Plan(args) => replicate_plan_command(&args),
//        Command::Apply(args) => replicate_apply_command(&args),
//    }
//}

fn main() {
    let url: String = "https://github.com/canonical/chisel".to_string();
    let version: String = "v1.2.0".to_string();
    let project_root: PathBuf = PathBuf::from(format!{
        "{}/source-wand-projects/", std::env::var("HOME").unwrap()
    });
    let module_name: String = "github.com/canonical/chisel".to_string();
    let mut graph: Graph<DependencyTreeNodeGo, String> = Graph::new();
    let graph = Arc::new(Graph::new());
    parse_dependency(&url, &version, &project_root, &module_name, Arc::clone(&graph)); 
    graph.print_dependencies();
}
