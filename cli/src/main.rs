use clap::{Parser, Subcommand};
use commands::deptree::{deptree_command, DeptreeArgs};

mod commands;

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[command(about = "Find the dependency tree of a project.")]
    Deptree(DeptreeArgs),
}

fn execute_command() -> Result<(), String> {
    match Cli::parse().command {
        Command::Deptree(args) => {
            deptree_command(&args)
        }
    }
}

fn main() {
    if let Err(e) = execute_command() {
        eprintln!("{}", e);
    }
}
