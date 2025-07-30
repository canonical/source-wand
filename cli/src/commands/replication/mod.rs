use anyhow::Result;
use clap::{command, Parser, Subcommand};

use crate::commands::replication::{apply::{replicate_apply_command, ReplicationApplyArgs}, init::{replicate_init_command, ReplicationInitArgs}, plan::{replicate_plan_command, ReplicationPlanArgs}};

pub mod init;
pub mod plan;
pub mod apply;

#[derive(Debug, Parser)]
pub struct ReplicationArgs {
    #[command(subcommand)]
    pub command: ReplicationCommand,
}

#[derive(Debug, Subcommand)]
pub enum ReplicationCommand {
    #[command(about = "Initialize a new deep replication project")]
    Init(ReplicationInitArgs),

    #[command(about = "Plan a deep replication and validate the replication is possible")]
    Plan(ReplicationPlanArgs),

    #[command(about = "Apply the deep replication plan")]
    Apply(ReplicationApplyArgs),
}

pub fn replicate_command(args: &ReplicationArgs) -> Result<()> {
    match &args.command {
        ReplicationCommand::Init(args) => replicate_init_command(args),
        ReplicationCommand::Plan(args) => replicate_plan_command(args),
        ReplicationCommand::Apply(args) => replicate_apply_command(args),
    }
}
