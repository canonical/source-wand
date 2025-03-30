use std::collections::HashMap;

use source_wand_common::{project::Project, project_manipulator::project_manipulator::{AnyProjectManipulator, ProjectManipulator}};

use crate::dependency_tree_node::DependencyTreeNode;

pub fn generate_go_dependency_tree(project_manipulator: &AnyProjectManipulator) -> Result<DependencyTreeNode, String> {
    let graph_raw: String = project_manipulator.run_shell("go mod graph".to_string())?;
    let mut root: Option<String> = None;
    let mut projects_map: HashMap<String, Project> = HashMap::new();
    let mut dep_map: HashMap<String, Vec<String>> = HashMap::new();

    for line in graph_raw.lines() {
        let tokens: Vec<&str> = line.split(" ").collect();

        let from: &&str = tokens.get(0).ok_or("Invalid graph line, expected \"from\" address")?;
        let to: &&str   = tokens.get(1).ok_or("Invalid graph line, expected \"to\" address")?;

        let from_project: Project = parse_dependency(&from)?;
        let to_project: Project   = parse_dependency(&to)?;

        let from: String = make_go_dependency_string(&from_project);
        let to: String   = make_go_dependency_string(&to_project);

        if root.is_none() {
            root = Some(make_go_dependency_string(&from_project));
        }

        projects_map.insert(from.clone(), from_project);
        projects_map.insert(to.clone(), to_project);

        store_dependency_of(&from, &to, &mut dep_map);
    }

    if root.is_none() {
        return Err("()".to_string());
    }

    let root: DependencyTreeNode = DependencyTreeNode::new(
        projects_map.get(&root.unwrap()).unwrap().clone(),
        Vec::new()
    );

    build_tree(&root, &projects_map, &mut dep_map)?.ok_or("Could not build dependency tree.".to_string())
}

fn build_tree(
    tree: &DependencyTreeNode,
    projects_map: &HashMap<String, Project>,
    dep_map: &mut HashMap<String, Vec<String>>,
) -> Result<Option<DependencyTreeNode>, String> {
    let dependencies: Vec<Project> = get_dependencies_of(
        &make_go_dependency_string(&tree.project),
        projects_map,
        dep_map
    )?;

    if dependencies.len() == 0 {
        return Ok(Some(tree.clone()));
    }

    let dependencies: Vec<Box<DependencyTreeNode>> = dependencies.iter()
        .map(
            |dependency| build_tree(
                &DependencyTreeNode::new(dependency.clone(), Vec::new()),
                projects_map,
                dep_map
            )
        )
        .filter(|dependency| dependency.is_ok())
        .filter_map(|dependency| dependency.unwrap())
        .map(|dependency| Box::new(dependency))
        .collect();

    Ok(Some(DependencyTreeNode::new(tree.project.clone(), dependencies)))
}

fn parse_dependency(raw: &str) -> Result<Project, String> {
    let tokens: Vec<&str> = raw.split("@").collect();

    let path: String    = tokens.get(0).ok_or("Invalid dependency format, expected \"path\"")?.to_string();
    let version: String = tokens.get(1).unwrap_or_else(|| &"").to_string();

    let name: String = path.rsplit("/").next().ok_or("Invalid path format, expected \"name\"")?.to_string();

    Ok(Project::new(name, version, "".to_string(), path))
}

fn store_dependency_of(of: &String, dependency: &String, dep_map: &mut HashMap<String, Vec<String>>) {
    if !dep_map.contains_key(of) {
        dep_map.insert(of.clone(), Vec::new());
    }

    dep_map.get_mut(of.as_str()).unwrap().push(dependency.clone());
}

fn get_dependencies_of(
    of: &String,
    projects_map: &HashMap<String, Project>,
    dep_map: &HashMap<String, Vec<String>>,
) -> Result<Vec<Project>, String> {
    let keys: &Vec<String> = dep_map.get(of).ok_or(format!("Dependencies {} not found.", of))?;
    let dependencies: Vec<Project> = keys.iter()
        .filter_map(|dependency| projects_map.get(dependency))
        .map(|dependency| dependency.clone())
        .collect();

    Ok(dependencies)
}

fn make_go_dependency_string(dependency: &Project) -> String {
    format!("{}@{}", dependency.repository, dependency.version)
}
