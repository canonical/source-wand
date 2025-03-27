use source_wand_common::{project::Project, project_manipulator::project_manipulator::{AnyProjectManipulator, ProjectManipulator}};
use crate::dependency_tree_node::DependencyTreeNode;

#[derive(Debug)]
struct DepthAnnotatedNode {
    node: DependencyTreeNode,
    depth: u32,
}

pub fn generate_java_maven_dependency_tree(project_manipulator: &AnyProjectManipulator) -> Result<DependencyTreeNode, String> {
    let raw_tree: String = project_manipulator.run_shell(
        "mvn dependency:tree".to_string()
    )?;

    let mut parsed_nodes = Vec::new();
    let root_project = parse_root_project(project_manipulator)?;

    parsed_nodes.push(DepthAnnotatedNode {
        depth: 0,
        node: DependencyTreeNode::new(root_project.clone(), Vec::new()),
    });

    for line in raw_tree.lines() {
        if line.is_empty() ||
           line.starts_with("[INFO] ---") ||
           line.starts_with("[INFO] Finished at: ") ||
           line.starts_with("Building ")
        {
            continue;
        }

        if line.contains(&format!("{}:{}", root_project.name, root_project.version)) {
            continue;
        }

        if let Ok((depth, dependency)) = parse_dependency_line(line) {
            parsed_nodes.push(DepthAnnotatedNode {
                depth,
                node: DependencyTreeNode::new(dependency, Vec::new()),
            });
        }
    }

    for i in (1..parsed_nodes.len()).rev() {
        let current_depth = parsed_nodes[i].depth;
        let mut parent_index = i - 1;

        while parent_index > 0 && parsed_nodes[parent_index].depth >= current_depth {
            parent_index -= 1;
        }

        let node = parsed_nodes[i].node.clone();
        parsed_nodes[parent_index].node.dependencies.push(Box::new(node));
    }

    Ok(parsed_nodes[0].node.clone())
}

fn parse_root_project(project_manipulator: &AnyProjectManipulator) -> Result<Project, String> {
    let artifact_id = project_manipulator.run_shell(
        "mvn help:evaluate -Dexpression=project.artifactId -q -DforceStdout".to_string()
    )?.trim().to_string();

    let version = project_manipulator.run_shell(
        "mvn help:evaluate -Dexpression=project.version -q -DforceStdout".to_string()
    )?.trim().to_string();

    Ok(Project::new(
        artifact_id,
        version,
        String::new(),
        String::new(),
    ))
}

fn parse_dependency_line(line: &str) -> Result<(u32, Project), String> {
    let clean_line = line.trim_start_matches("[INFO] ");
    let mut depth = 0;
    let mut chars = clean_line.chars().peekable();

    while let Some(c) = chars.peek() {
        match c {
            ' ' | '│' | '├' | '└' | '─' | '+' | '\\' => {
                depth += 1;
                chars.next();
            }
            _ => break,
        }
    }

    let depth = (depth / 3) as u32;

    let dep_str: String = chars.collect();
    let parts: Vec<&str> = dep_str.split(':').collect();

    if parts.len() < 4 {
        return Err(format!("Invalid dependency format: {}", dep_str));
    }

    let artifact_id = parts[1].trim().to_string();
    let version = parts[3].trim().to_string();

    Ok((depth, Project::new(
        artifact_id,
        version,
        String::new(),
        String::new(),
    )))
}
