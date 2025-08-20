use std::{
    collections::{HashMap, HashSet, VecDeque}, fs::create_dir_all, path::PathBuf, sync::{Arc, Mutex, MutexGuard}
};
use colorize::AnsiColor;
use rayon::prelude::*;
use source_wand_common::project_manipulator::local_project_manipulator::LocalProjectManipulator;
use anyhow::{anyhow, Result};
use uuid::Uuid;

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
    let node_map: HashMap<NodeId, Arc<TransformationNode>> =
        nodes.iter().map(|n| (n.id, Arc::clone(n))).collect();
    let completed: Arc<Mutex<HashSet<usize>>> = Arc::new(Mutex::new(HashSet::new()));
    let queue: Arc<Mutex<VecDeque<usize>>> = Arc::new(Mutex::new(
        nodes
            .iter()
            .filter(|node| node.dependencies.is_empty())
            .map(|node| node.id)
            .collect::<VecDeque<_>>(),
    ));
    let dependents: HashMap<NodeId, Vec<NodeId>> = {
        let mut map: HashMap<NodeId, Vec<NodeId>> = HashMap::new();
        for node in &nodes {
            for &dep in &node.dependencies {
                map.entry(dep).or_default().push(node.id);
            }
        }
        map
    };

    let error: Arc<Mutex<std::result::Result<(), anyhow::Error>>> = Arc::new(Mutex::new(Ok(())));

    loop {
        let batch: Vec<NodeId> = {
            let mut q: MutexGuard<'_, VecDeque<usize>> = queue.lock().unwrap();
            q.drain(..).collect()
        };

        if batch.is_empty() {
            break;
        }

        batch.par_iter().for_each(|&node_id| {
            if error.lock().unwrap().is_err() {
                return;
            }

            let node: &Arc<TransformationNode> = node_map.get(&node_id).unwrap();

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
                    let transformation_result: Result<Context> = node.transformation.apply(ctx);
                    if let Err(e) = transformation_result {
                        *error.lock().unwrap() = Err(e);
                        return;
                    }
                    else {
                        println!(
                            "{:<106} context: {}",
                            format!(
                                "{} {}",
                                "[execute]".to_string().green(),
                                node.transformation.get_name().blue(),
                            ),
                            node.workdesk,
                        );
                    }
                }
            } else {
                *error.lock().unwrap() = Err(anyhow::anyhow!("Missing context for workdesk {}", node.workdesk));
                return;
            }

            completed.lock().unwrap().insert(node_id);

            if let Some(deps) = dependents.get(&node_id) {
                for &dependent_id in deps {
                    let dependent: &Arc<TransformationNode> = node_map.get(&dependent_id).unwrap();
                    let ready: bool = dependent
                        .dependencies
                        .iter()
                        .all(|dep| completed.lock().unwrap().contains(dep));
                    if ready {
                        queue.lock().unwrap().push_back(dependent_id);
                    }
                }
            }
        });

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
