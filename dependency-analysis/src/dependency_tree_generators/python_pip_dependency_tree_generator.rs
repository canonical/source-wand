use anyhow::Result;
use source_wand_common::{project::Project, project_manipulator::project_manipulator::{AnyProjectManipulator, ProjectManipulator}};

use crate::dependency_tree_node::DependencyTreeNode;

pub fn generate_python_pip_dependency_tree(project_manipulator: &AnyProjectManipulator) -> Result<DependencyTreeNode> {
    let root_name: String = project_manipulator.run_shell("basename \"$PWD\"".to_string())?.trim().to_string();
    let raw_tree: String = project_manipulator.run_shell("pipgrip --requirements-file requirements.txt --tree".to_string())?;

    let mut parsed_tree: Vec<DepthAnnotatedDependencyTreeNode> = Vec::new();

    parsed_tree.push(
        DepthAnnotatedDependencyTreeNode {
            depth: 0,
            node: DependencyTreeNode::new(
                Project::new(root_name, String::new(), String::new(), String::new()),
                Vec::new(),
            )
        }
    );

    for line in raw_tree.lines() {
        let mut chars = line.chars().peekable();

        let mut depth: u32 = 1;
        loop {
            if let Some(character) = chars.peek() {
                if is_padding(&character) {
                  depth += 1;
                  chars.next();
                }
                else { break; }
            }
            else { break; }
        }

        let mut name: String = String::new();
        loop {
            if let Some(character) = chars.next() {
                if is_name_version_delimiter(&character) { break; }
                else { name.push(character); }
            }
            else {
                break;
            }
        }

        loop {
            if let Some(character) = chars.next() {
                if character == '(' { break; }
            }
            else { break; }
        }

        let mut version: String = String::new();
        loop {
            if let Some(character) = chars.next() {
                if character == ')' { break; }
                else { version.push(character); }
            }
            else { break; }
        }

        let license: String = String::new();
        let repository: String = String::new();

        let node: DependencyTreeNode = DependencyTreeNode::new(
            Project::new(name, version, license, repository),
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
        parsed_tree[parent_index].node.dependencies.push(Box::new(node));
    }

    Ok(parsed_tree[0].node.clone())
}

fn is_padding(character: &char) -> bool {
    character == &'│' || character == &'├' || character == &'─' || character == &'└' || character == &' '
}

fn is_name_version_delimiter(character: &char) -> bool {
    character == &'=' || character == &'>' || character == &'<' || character == &'~'
}

#[derive(Debug)]
struct DepthAnnotatedDependencyTreeNode {
    pub node: DependencyTreeNode,
    pub depth: u32,
}
