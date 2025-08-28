use std::{
    collections::{
        HashMap,
        HashSet,
    },
    fs::create_dir_all,
    path::PathBuf,
    sync::{
        Arc,
        Mutex,
        MutexGuard,
    }
};

use colorize::AnsiColor;
use rayon::prelude::*;
use anyhow::{anyhow, Error, Result};
use uuid::Uuid;

use source_wand_common::project_manipulator::local_project_manipulator::LocalProjectManipulator;

use crate::plan::{context::Context, transformation_node::{NodeId, TransformationNode}};

pub fn execute_plan(nodes: Vec<Arc<TransformationNode>>) -> Result<()> {
    let mut workdesk_contexts: HashMap<String, Context> = HashMap::new();
    for node in &nodes {
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
    }

    let context_map: Arc<Mutex<HashMap<String, Context>>> = Arc::new(Mutex::new(workdesk_contexts));
    let completed: Arc<Mutex<HashSet<NodeId>>> = Arc::new(Mutex::new(HashSet::new()));

    let error: Arc<Mutex<Result<(), Error>>> = Arc::new(Mutex::new(Ok(())));

    while completed.lock().unwrap().len() < nodes.len() {
        let ready_nodes: Vec<Arc<TransformationNode>> = nodes
            .iter()
            .filter(
                |node|
                    !completed.lock().unwrap().contains(&node.id) &&
                    node.dependencies
                        .iter()
                        .all(
                            |dependency|
                            completed
                                .lock()
                                .unwrap()
                                .contains(dependency)
                        )
            )
            .map(|node| node.clone())
            .collect();

        ready_nodes
            .par_iter()
            .for_each(
                |node| {
                    let ctx: Option<Context> = {
                        let mut contexts: MutexGuard<'_, HashMap<String, Context>> = context_map.lock().unwrap();
                        contexts.get_mut(&node.workdesk).cloned()
                    };

                    if let Some(ctx) = ctx {
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

                    completed.lock().unwrap().insert(node.id);
                }
            );
        
        if error.lock().unwrap().is_err() {
            break;
        }
    }

    let error: MutexGuard<'_, std::result::Result<(), anyhow::Error>> = error.lock().unwrap();
    match &*error {
        Ok(()) => Ok(()),
        Err(e) => Err(anyhow!(e.to_string())),
    }
}
