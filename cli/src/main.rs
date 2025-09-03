use std::{env, path::PathBuf, sync::Arc};

use clap::{Parser, Subcommand};
use commands::dependencies::DependenciesArgs;
use source_wand_common::utils::read_yaml_file::read_yaml_file;
use source_wand_dependency_analysis::dependency_tree_generators::{go_dependency_tree_generator_andrew::parse_dependency, go_depenendency_tree_struct::Graph};
use source_wand_replication::model::replication_manifest::ReplicationManifest;

use crate::commands::replication::{init::{replicate_init_command, ReplicationInitArgs}, plan::replication_plan_andrew_go, ReplicationArgs};

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
    let project_root: PathBuf = PathBuf::from(format!{
        "{}/source-wand-projects/", std::env::var("HOME").unwrap()
    });
    let module_name: String = "github.com/canonical/chisel".to_string();
    //let graph = Arc::new(Graph::new());
    //parse_dependency(&url, &version, &project_root, &module_name, Arc::clone(&graph)); 
    //graph.print_graph();
    //println!("Final map size: {}", graph.nodes.len());
    //println!("{:#?}", graph.get_node_list());

    let replication_args = ReplicationInitArgs {};

    let _ = replicate_init_command(&replication_args);

    // Replication Plan Andrew Go
    let rep_plan = replication_plan_andrew_go(&url, &version, &project_root, &module_name).unwrap();
    println!("{:#?}", rep_plan);



}