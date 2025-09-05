use std::{fs::remove_dir_all, path::PathBuf, sync::Arc};

use anyhow::Result;
use clap::Parser;
use colorize::AnsiColor;
use source_wand_concurrent_executor::{
    executor::execute_graph,
    transformation_node::TransformationNode
};
use source_wand_replication::{
    model::replication_plan::ReplicationPlan,
    plan::planner::plan_replication
};

#[derive(Debug, Parser)]
pub struct ApplyArgs;

pub fn replicate_apply_command(_args: &ApplyArgs) -> Result<()> {
    println!(
        "{} analyzing the origin project's dependency tree",
        "[plan]".green(),
    );

    let replication_plan: ReplicationPlan = plan_replication()?;

    let execution_graph: Vec<Arc<TransformationNode>> = replication_plan.to_execution_graph();
    execute_graph(execution_graph)?;

    remove_dir_all(PathBuf::from("./source-wand")).ok();

    Ok(())
}
