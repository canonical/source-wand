use std::{path::PathBuf, sync::Arc};

use clap::{Parser, Subcommand};
use commands::dependencies::DependenciesArgs;
use source_wand_dependency_analysis::dependency_tree_generators::{go_dependency_tree_generator_andrew::parse_dependency, go_depenendency_tree_struct::Graph};

use crate::commands::replication::ReplicationArgs;

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

    #[command(about = "Replicate a project along with its dependencies")]
    Replication(ReplicationArgs)
}

//fn main() -> Result<()> {
//    match Cli::parse().command {
//        Command::Dependencies(args) => dependencies_command(&args),
//        Command::Replication(args) => replicate_command(&args),
//    }
//}

fn main() {
    let url: String = "https://github.com/canonical/chisel".to_string();
    let version: String = "v1.2.0".to_string();
    let project_root: PathBuf = PathBuf::from("/home/andrew/source-wand-projects");
    let module_name: String = "github.com/canonical/chisel".to_string();
    let graph = Arc::new(Graph::new());
    parse_dependency(&url, &version, &project_root, &module_name, Arc::clone(&graph)); 
    graph.print_dependencies();
    //println!("{:#?}", graph);
    //println!("{}", graph.to_dot());
}