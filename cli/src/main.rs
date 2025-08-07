use anyhow::Result;
use clap::{Parser, Subcommand};
use commands::{
    dependencies::{
        dependencies_command,
        DependenciesArgs
    }
};

use crate::commands::replication::{replicate_command, ReplicationArgs};

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

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Dependencies(args) => dependencies_command(&args),
        Command::Replication(args) => replicate_command(&args),
    }
}
