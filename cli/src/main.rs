use anyhow::Result;
use clap::{Parser, Subcommand};
use commands::{
    dependencies::{
        dependencies_command,
        DependenciesArgs
    }
};

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
    replication::{
        replication_command,
        ReplicationArgs
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

    #[command(about = "[DEPRECATED] Replicate a project")]
    Replication(ReplicationArgs),
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Dependencies(args) => dependencies_command(&args),
        Command::Init(args) => replicate_init_command(&args),
        Command::Plan(args) => replicate_plan_command(&args),
        Command::Apply(args) => replicate_apply_command(&args),
        Command::Replication(args) => replication_command(&args),
    }
}
