use std::{
    collections::HashMap, fs, path::PathBuf, sync::{Arc, Mutex},
};

use anyhow::{Error, Result};
use serde::Deserialize;
use source_wand_common::{
    project::Project, project_manipulator::project_manipulator::ProjectManipulator,
};

use crate::dependency_tree_node::DependencyTreeNode;

#[derive(Debug, Deserialize)]
struct Bom {
    metadata: BomMetadata,
    components: Vec<Component>,
    dependencies: Vec<BomDependency>,
}

#[derive(Debug, Deserialize)]
struct BomMetadata {
    component: Component,
}

#[derive(Debug, Deserialize)]
struct Property {
    name: String,
    value: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Component {
    #[serde(rename = "bom-ref")]
    bom_ref: String,
    name: String,
    version: Option<String>,
    #[serde(default)]
    properties: Vec<Property>,
}

#[derive(Debug, Deserialize)]
struct BomDependency {
    #[serde(rename = "ref")]
    ref_: String,
    #[serde(rename = "dependsOn")]
    depends_on: Vec<String>,
}

pub fn generate_cdxgen_dependency_tree(
    project_manipulator: &dyn ProjectManipulator,
    _language: Option<&str>,
) -> Result<Arc<Mutex<DependencyTreeNode>>> {
    project_manipulator.run_shell(
        format!(
            "cdxgen -o bom.source-wand.json --output-format json",
        )
    )?;

    let bom_path: PathBuf = project_manipulator.get_working_directory().join("bom.source-wand.json");

    let bom_raw: String = fs::read_to_string(&bom_path)
        .map_err(|e| Error::msg(format!("Failed to read bom.json: {}", e)))?;

    fs::remove_file(&bom_path)
        .map_err(|e| Error::msg(format!("Failed to remove bom.json: {}", e)))?;

    let bom: Bom = serde_json::from_str(&bom_raw)
        .map_err(
            |e| {
                Error::msg(
                    format!(
                        "Failed to parse BOM JSON: {}\n\nHere is the BOM in question:\n{}",
                        e,
                        serde_json::to_string_pretty(
                            &serde_json::from_str::<serde_json::Value>(&bom_raw.as_str()).unwrap()
                        ).unwrap()
                    )
                )
            }
        )?;

    let mut component_map: HashMap<String, Project> = HashMap::new();

    let root_project: Project = Project::new(
        bom.metadata.component.name,
        bom.metadata.component.version.unwrap_or_else(|| "Not found".to_string()),
        String::new(),
        String::new(),
        None,
        None,
    );
    component_map.insert(bom.metadata.component.bom_ref.clone(), root_project);

    for component in bom.components {
        let mut group_id: Option<String> = Some(String::new());
        let mut artifact_id: Option<String> = Some(String::new());

        for prop in &component.properties {
            if prop.name == "group_id" {
                group_id = prop.value.clone();
            } else if prop.name == "artifact_id" {
                artifact_id = prop.value.clone();
            }
        }

        let project: Project = Project::new(
            component.name,
            component.version.unwrap_or_else(|| "Not found".to_string()),
            group_id.unwrap_or_default(),
            artifact_id.unwrap_or_default(),
            None,
            None,
        );
        component_map.insert(component.bom_ref, project);
    }

    let mut dependency_map: HashMap<String, Vec<String>> = HashMap::new();

    for dep in bom.dependencies {
        dependency_map.insert(dep.ref_, dep.depends_on);
    }

    let root_bom_ref: String = bom.metadata.component.bom_ref;
    let root_node: Arc<Mutex<DependencyTreeNode>> = build_node_iterative(&root_bom_ref, &component_map, &dependency_map)?;

    Ok(root_node)
}

struct StackFrame<'a> {
    component_ref: String,
    child_refs_iter: std::slice::Iter<'a, String>,
    children: Vec<Arc<Mutex<DependencyTreeNode>>>,
}

fn build_node_iterative(
    root_ref: &str,
    component_map: &HashMap<String, Project>,
    dependency_map: &HashMap<String, Vec<String>>,
) -> Result<Arc<Mutex<DependencyTreeNode>>> {
    let mut stack: Vec<StackFrame> = Vec::new();

    let root_children = dependency_map.get(root_ref)
        .map(|v| v.as_slice())
        .unwrap_or(&[]);

    stack.push(StackFrame {
        component_ref: root_ref.to_owned(),
        child_refs_iter: root_children.iter(),
        children: Vec::new(),
    });

    let mut result: Option<Arc<Mutex<DependencyTreeNode>>> = None;

    while let Some(top) = stack.last_mut() {
        if let Some(next_child_ref) = top.child_refs_iter.next() {
            let child_children = dependency_map.get(next_child_ref.as_str())
                .map(|v| v.as_slice())
                .unwrap_or(&[]);

            stack.push(StackFrame {
                component_ref: next_child_ref.to_owned(),
                child_refs_iter: child_children.iter(),
                children: Vec::new(),
            });
        } else {
            let frame: StackFrame<'_> = stack.pop().unwrap();

            let project: Project = component_map.get(&frame.component_ref)
                .ok_or_else(|| Error::msg(format!("Component with ref '{}' not found in map", &frame.component_ref)))?
                .clone();

            let node: Arc<Mutex<DependencyTreeNode>> = Arc::new(
                Mutex::new(
                    DependencyTreeNode::new(project,frame.children)
                )
            );

            if let Some(parent) = stack.last_mut() {
                parent.children.push(node);
            } else {
                result = Some(node);
            }
        }
    }

    result.ok_or_else(|| Error::msg("Failed to build dependency tree"))
}
