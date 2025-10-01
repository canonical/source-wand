use anyhow::{bail, Result};
use clap::{Parser, Subcommand};
use colorize::AnsiColor;

#[derive(Debug, Parser)]
pub struct ReplicationArgs {
    #[command(subcommand)]
    command: ReplicationCommand,
}

#[derive(Debug, Subcommand)]
pub enum ReplicationCommand {
    #[command(about = "[MOVED] Initialize a new deep replication project")]
    Init,
    #[command(about = "[MOVED] Plan a deep replication and validate the replication is possible")]
    Plan,
    #[command(about = "[MOVED] Apply the deep replication plan")]
    Apply,
}

pub fn replication_command(args: &ReplicationArgs) -> Result<()> {
    match args.command {
        ReplicationCommand::Init => bail!(
            "replication commands were moved\n\n{} was deprecated\n{} should now be used instead\n",
            " $ source-wand replication init".red().italic(),
            " $ source-wand init".green().italic(),
        ),
        ReplicationCommand::Plan => bail!(
            "replication commands were moved\n\n{} was deprecated\n{} should now be used instead\n",
            " $ source-wand replication plan".red().italic(),
            " $ source-wand plan".green().italic(),
        ),
        ReplicationCommand::Apply => bail!(
            "replication commands were moved\n\n{} was deprecated\n{} should now be used instead\n",
            " $ source-wand replication apply".red().italic(),
            " $ source-wand apply".green().italic(),
        ),
    }
}
