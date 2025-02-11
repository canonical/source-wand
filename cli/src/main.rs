use clap::{Parser, Subcommand};
use commands::dependencies::{dependencies_command, DependenciesArgs};

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
}

fn execute_command() -> Result<(), String> {
    match Cli::parse().command {
        Command::Dependencies(args) => {
            dependencies_command(&args)
        }
    }
}

fn main() {
    if let Err(e) = execute_command() {
        eprintln!("{}", e);
    }
}
