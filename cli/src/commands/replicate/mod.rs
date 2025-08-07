use anyhow::Result;
use clap::{command, Parser, Subcommand};

use crate::commands::replicate::{apply::{replicate_apply_command, ReplicateApplyArgs}, init::{replicate_init_command, ReplicateInitArgs}, plan::{replicate_plan_command, ReplicatePlanArgs}};

pub mod init;
pub mod plan;
pub mod apply;

#[derive(Debug, Parser)]
pub struct ReplicateArgs {
    #[command(subcommand)]
    pub command: ReplicateCommand,
}

#[derive(Debug, Subcommand)]
pub enum ReplicateCommand {
    #[command(about = "Initialize a new deep replication project")]
    Init(ReplicateInitArgs),

    #[command(about = "Plan a deep replication and validate the replication is possible")]
    Plan(ReplicatePlanArgs),

    #[command(about = "Apply the deep replication plan")]
    Apply(ReplicateApplyArgs),
}

pub fn replicate_command(args: &ReplicateArgs) -> Result<()> {
    match &args.command {
        ReplicateCommand::Init(args) => replicate_init_command(args),
        ReplicateCommand::Plan(args) => replicate_plan_command(args),
        ReplicateCommand::Apply(args) => replicate_apply_command(args),
    }
}
