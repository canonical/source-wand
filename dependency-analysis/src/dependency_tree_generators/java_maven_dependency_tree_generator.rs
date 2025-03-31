use regex::Regex;
use source_wand_common::{project::Project, project_manipulator::project_manipulator::{AnyProjectManipulator, ProjectManipulator}};
use crate::dependency_tree_node::DependencyTreeNode;

pub fn generate_java_maven_dependency_tree(project_manipulator: &AnyProjectManipulator) -> Result<DependencyTreeNode, String> {
    let logs: String = project_manipulator.run_shell(
        "mvn dependency:tree".to_string()
    )?;

    let (root, branches) = extract_tree_from_logs(&logs)?;

    let mut anotated_tree: Vec<DepthAnnotatedDependencyTreeNode> = anotate_tree(&root, &branches);

    for i in (1..anotated_tree.len()).rev() {
        let depth: u32 = anotated_tree[i].depth;

        let mut parent_index: usize = i - 1;
        while parent_index > 0 && anotated_tree[parent_index].depth >= depth {
            parent_index -= 1;
        }

        let node: DependencyTreeNode = anotated_tree[i].node.clone();
        anotated_tree[parent_index].node.dependencies.push(Box::new(node));
    }

    Ok(anotated_tree[0].node.clone())
}

#[derive(Debug)]
struct DepthAnnotatedDependencyTreeNode {
    pub node: DependencyTreeNode,
    pub depth: u32,
}

fn anotate_tree(root: &String, branches: &Vec<String>) -> Vec<DepthAnnotatedDependencyTreeNode> {
    let mut nodes: Vec<DepthAnnotatedDependencyTreeNode> = Vec::new();

    nodes.push(DepthAnnotatedDependencyTreeNode {
        node: DependencyTreeNode::new(parse_project(root), Vec::new()),
        depth: 0,
    });

    for branch in branches {
        let (branch, depth) = compute_branch_depth(branch);

        if branch.is_empty() {
            continue;
        }

        let node: DependencyTreeNode = DependencyTreeNode::new(parse_project(&branch), Vec::new());
        nodes.push(DepthAnnotatedDependencyTreeNode { node, depth });
    }

    nodes
}

fn parse_project(raw: &String) -> Project {
    let tokens: Vec<String> = raw.split(":").map(|token| token.to_string()).collect();
    let name: String = format!("{}.{}", tokens.get(0).unwrap(), tokens.get(1).unwrap());
    let version: String = tokens.get(3).unwrap().to_string();
    let license: String = String::new();
    let repository: String = String::new();

    Project::new(name, version, license, repository)
}

fn compute_branch_depth(branch: &String) -> (String, u32) {
    let initial_length: usize = branch.len();

    let branch_prefix_expr: Regex = Regex::new(r"^[\| +\\-]+").unwrap();
    let striped_branch: String = branch_prefix_expr.replace(&branch, String::new()).into_owned();

    let final_length: usize = striped_branch.len();
    let depth: u32 = (initial_length - final_length) as u32;

    (striped_branch, depth)
}

fn extract_tree_from_logs(logs: &String) -> Result<(String, Vec<String>), String> {
    let record_begin_marker: &str = "[\u{1b}[1;34mINFO\u{1b}[m] \u{1b}[1m--- ";
    let record_end_marker: &str = "[\u{1b}[1;34mINFO\u{1b}[m] \u{1b}[1m-";
    let prefix_to_strip: &str = "[\u{1b}[1;34mINFO\u{1b}[m] ";

    let mut lines: Vec<String> = Vec::new();
    let mut record: bool = false;

    for line in logs.lines() {
        if line.starts_with(record_end_marker) {
            record = false;
        }

        if record {
            if let Some(line) = line.strip_prefix(prefix_to_strip) {
                lines.push(line.to_string());
            }
        }

        if line.starts_with(record_begin_marker) {
            record = true;
        }
    }

    let (root, branches) = lines.split_first()
        .ok_or("Invalid tree format. Could not separate root from branches.")?;

    Ok(
        (
            root.to_string(),
            branches.iter()
                    .map(|branch| branch.to_string())
                    .collect()
        )
    )
}
