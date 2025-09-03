use std::sync::{Arc, Mutex};

use anyhow::{Error, Result};
use source_wand_common::{project::Project, project_manipulator::project_manipulator::{AnyProjectManipulator, ProjectManipulator}};

use crate::dependency_tree_node::DependencyTreeNode;

pub fn generate_rust_cargo_dependency_tree(
    project_manipulator: &AnyProjectManipulator
) -> Result<Arc<Mutex<DependencyTreeNode>>> {
    let raw_tree: String = project_manipulator.run_shell("cargo tree --prefix depth --format \" ;; {p} ;; {l} ;; {r}\"".to_string())?;
    let mut parsed_tree: Vec<DepthAnnotatedDependencyTreeNode> = Vec::new();

    for line in raw_tree.lines() {
        let tokens: Vec<String> = line.split(";;")
            .map(str::trim)
            .map(str::to_string)
            .collect();
        let name_and_version: Vec<String> = tokens[1].split(" ")
            .map(str::trim)
            .map(str::to_string)
            .collect();

        let depth: u32 = tokens[0].parse::<u32>().map_err(|_| Error::msg("Unable to parse depth"))?;
        let name: String = name_and_version[0].clone();
        let version: String = name_and_version[1].clone();
        let license: String = tokens[2].clone();
        let repository: String = tokens[3].clone();

        let node: DependencyTreeNode = DependencyTreeNode::new(
            Project::new(name, version, license, repository, None, None),
            Vec::new(),
        );

        parsed_tree.push( DepthAnnotatedDependencyTreeNode { depth, node } );
    }

    for i in (1..parsed_tree.len()).rev() {
        let depth: u32 = parsed_tree[i].depth;

        let mut parent_index: usize = i - 1;
        while parent_index > 0 && parsed_tree[parent_index].depth >= depth {
            parent_index -= 1;
        }

        let node: DependencyTreeNode = parsed_tree[i].node.clone();
        parsed_tree[parent_index].node.dependencies.push(Arc::new(Mutex::new(node)));
    }

    Ok(Arc::new(Mutex::new(parsed_tree[0].node.clone())))
}

#[derive(Debug)]
struct DepthAnnotatedDependencyTreeNode {
    pub node: DependencyTreeNode,
    pub depth: u32,
}
