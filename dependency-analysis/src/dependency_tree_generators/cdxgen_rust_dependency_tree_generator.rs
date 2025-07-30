use std::{
    collections::HashMap, fs, path::PathBuf
};

use anyhow::{Error, Result};
use serde::Deserialize;
use source_wand_common::{
    project::Project, project_manipulator::project_manipulator::ProjectManipulator
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
    version: String,
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

pub fn generate_cdxgen_rust_dependency_tree(
    project_manipulator: &dyn ProjectManipulator,
) -> Result<DependencyTreeNode> {
    project_manipulator.run_shell("cdxgen -t rust -o bom.source-wand.json --output-format json".to_string())?;

    let bom_path: PathBuf = project_manipulator.get_working_directory().join("bom.source-wand.json");

    let bom_raw = fs::read_to_string(&bom_path)
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
        bom.metadata.component.version,
        String::new(),
        String::new(),
        None,
        None,
    );
    component_map.insert(bom.metadata.component.bom_ref.clone(), root_project);

    for component in bom.components {
        let mut group_id = Some(String::new());
        let mut artifact_id = Some(String::new());

        for prop in &component.properties {
            if prop.name == "group_id" {
                group_id = prop.value.clone();
            } else if prop.name == "artifact_id" {
                artifact_id = prop.value.clone();
            }
        }

        let project = Project::new(
            component.name,
            component.version,
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

    let root_bom_ref = bom.metadata.component.bom_ref;
    let root_node = build_node(&root_bom_ref, &component_map, &dependency_map)?;

    Ok(*root_node)
}

fn build_node(
    component_ref: &str,
    component_map: &HashMap<String, Project>,
    dependency_map: &HashMap<String, Vec<String>>,
) -> Result<Box<DependencyTreeNode>> {
    let project = component_map
        .get(component_ref)
        .ok_or_else(|| Error::msg(format!("Component with ref '{}' not found in map", component_ref)))?
        .clone();

    let binding = Vec::new();
    let direct_dependencies_refs = dependency_map
        .get(component_ref)
        .unwrap_or(&binding);

    let mut child_dependencies: Vec<Box<DependencyTreeNode>> = Vec::new();

    for dep_ref in direct_dependencies_refs {
        let child_node = build_node(dep_ref, component_map, dependency_map)?;
        child_dependencies.push(child_node);
    }

    Ok(Box::new(DependencyTreeNode::new(project, child_dependencies)))
}
