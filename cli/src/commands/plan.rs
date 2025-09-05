use anyhow::Result;
use clap::Parser;
use source_wand_replication::plan::planner::plan_replication;

#[derive(Debug, Parser)]
pub struct PlanArgs;

pub fn replicate_plan_command(_args: &PlanArgs) -> Result<()> {
    plan_replication()?;
    Ok(())
}
