use clap::{Parser, Subcommand};
use commands::{
    compare::{
        compare_command,
        CompareArgs
    },
    dependencies::{
        dependencies_command,
        DependenciesArgs
    },
    init::{
        init_command,
        InitArgs
    },
    mirror_dependencies::{
        mirror_dependencies_command,
        MirrorDependenciesArgs
    }
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

    #[command(about = "Compare dependency lists")]
    Compare(CompareArgs),

    #[command(about = "Mirror dependencies")]
    MirrorDependencies(MirrorDependenciesArgs),

    #[command(about = "Initialize the onboarding of a project")]
    Init(InitArgs),
}

fn execute_command() -> Result<(), String> {
    match Cli::parse().command {
        Command::Dependencies(args) => dependencies_command(&args),
        Command::Compare(args) => compare_command(&args),
        Command::MirrorDependencies(args) => mirror_dependencies_command(&args),
        Command::Init(args) => init_command(&args),
    }
}

fn main() {
    if let Err(e) = execute_command() {
        eprintln!("{}", e);
    }
}
