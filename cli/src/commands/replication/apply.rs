use std::{fs::remove_dir_all, path::PathBuf, sync::Arc};

use anyhow::Result;
use clap::Parser;
use colorize::AnsiColor;
use source_wand_replication::{
    apply::plan_executor::execute_plan,
    model::replication_plan::ReplicationPlan,
    plan::transformation_node::TransformationNode
};

use crate::commands::replication::plan::plan_replication;

#[derive(Debug, Parser)]
pub struct ReplicationApplyArgs;

pub fn replicate_apply_command(_args: &ReplicationApplyArgs) -> Result<()> {
    println!(
        "{} analyzing the origin project's dependency tree",
        "[plan]".green(),
    );

    let replication_plan: ReplicationPlan = plan_replication()?;

    let execution_graph: Vec<Arc<TransformationNode>> = replication_plan.to_execution_graph();
    execute_plan(execution_graph)?;

    remove_dir_all(PathBuf::from("./source-wand")).ok();

    Ok(())
}
