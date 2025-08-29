use std::{
    fs::create_dir_all,
    io,
    path::PathBuf,
    sync::{
        Arc,
        Mutex,
        MutexGuard,
    }
};

use colorize::AnsiColor;
use dashmap::{mapref::one::RefMut, DashMap};
use rayon::prelude::*;
use anyhow::{anyhow, Error, Result};
use uuid::Uuid;

use source_wand_common::project_manipulator::local_project_manipulator::LocalProjectManipulator;

use crate::{
    context::Context, execution_status_tracker::ExecutionProgressTracker, transformation_node::TransformationNode
};

pub fn execute_graph(nodes: Vec<Arc<TransformationNode>>) -> Result<()> {
    let workdesk_contexts: DashMap<String, Context> = DashMap::new();
    nodes.par_iter().map(
        |node| {
            if !workdesk_contexts.contains_key(&node.workdesk) {
                let uuid: Uuid = Uuid::new_v4();
                let source_directory: PathBuf = PathBuf::from(format!("./source-wand/{}", uuid));

                create_dir_all(&source_directory)?;

                let sh: LocalProjectManipulator = LocalProjectManipulator::new(
                    source_directory,
                    false,
                );

                workdesk_contexts.insert(node.workdesk.clone(), Context::new(sh));
            }

            Ok(())
        }
    ).collect::<Result<(), io::Error>>()?;

    let context_map: Arc<DashMap<String, Context>> = Arc::new(workdesk_contexts);

    let execution_progress_tracker: Arc<Mutex<ExecutionProgressTracker>> = Arc::new(Mutex::new(ExecutionProgressTracker::new()));

    let error: Arc<Mutex<Result<(), Error>>> = Arc::new(Mutex::new(Ok(())));

    while execution_progress_tracker.lock().unwrap().count_completed() < nodes.len() {
        schedule_ready_nodes(&nodes, &context_map, &execution_progress_tracker, &error);

        if error.lock().unwrap().is_err() {
            break;
        }
    }

    let error: MutexGuard<'_, Result<(), anyhow::Error>> = error.lock().unwrap();
    match &*error {
        Ok(()) => Ok(()),
        Err(e) => Err(anyhow!(e.to_string())),
    }
}

fn handle_node_execution(
    node: &Arc<TransformationNode>,
    nodes: &Vec<Arc<TransformationNode>>,
    context_map: &Arc<DashMap<String, Context>>,
    execution_progress_tracker: &Arc<Mutex<ExecutionProgressTracker>>,
    error: &Arc<Mutex<Result<(), Error>>>,
) {
    let ctx: Option<RefMut<'_, String, Context>> = context_map.get_mut(&node.workdesk);

    if let Some(ctx) = ctx {
        let ctx: Context = ctx.value().to_owned();

        if let Some(reason) = node.transformation.should_skip(&ctx) {
            println!(
                "{:<120} context: {}",
                format!(
                    "{} {} {}",
                    "[skip]".to_string().yellow(),
                    node.transformation.get_name().blue(),
                    reason.italic(),
                ),
                node.workdesk,
            );
        }
        else {
            let transformation_result: Result<Option<String>> = node.transformation.apply(ctx);
            match transformation_result {
                Ok(message) => {
                    let message: String = message.unwrap_or_default();

                    println!(
                        "{:<120} context: {}",
                        format!(
                            "{} {} {}",
                            "[execute]".to_string().green(),
                            node.transformation.get_name().blue(),
                            message.italic(),
                        ),
                        node.workdesk,
                    );
                },
                Err(e) => {
                    *error.lock().unwrap() = Err(e);
                    return;
                }
            }
        }
    } else {
        *error.lock().unwrap() = Err(anyhow::anyhow!("Missing context for workdesk {}", node.workdesk));
        return;
    }

    execution_progress_tracker.lock().unwrap().complete(node.id);
    schedule_ready_nodes(nodes, context_map, execution_progress_tracker, error);
}

fn schedule_ready_nodes(
    nodes: &Vec<Arc<TransformationNode>>,
    context_map: &Arc<DashMap<String, Context>>,
    execution_progress_tracker: &Arc<Mutex<ExecutionProgressTracker>>,
    error: &Arc<Mutex<Result<(), Error>>>,
) {
    let ready_nodes: Vec<Arc<TransformationNode>> = nodes
        .iter()
        .filter(
            |node| {
                if execution_progress_tracker.lock().unwrap().is_available(&node.id) &&
                    node.dependencies
                        .iter()
                        .all(|dependency| execution_progress_tracker.lock().unwrap().has_completed(&dependency))
                {
                    execution_progress_tracker.lock().unwrap().reserve(node.id);
                    true
                }
                else {
                    false
                }
            }
        )
        .map(|node| node.clone())
        .collect();

    ready_nodes
        .par_iter()
        .for_each(
            |node| {
                handle_node_execution(node, &nodes, &context_map, &execution_progress_tracker, &error);
            }
        );
}
