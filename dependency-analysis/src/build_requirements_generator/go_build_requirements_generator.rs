use std::{collections::HashMap, sync::{Arc, Mutex}};

use anyhow::{Ok, Result};
use source_wand_common::{project::Project, project_manipulator::project_manipulator::ProjectManipulator};

use crate::{
    dependency_tree_node::DependencyTreeNode,
    unique_dependencies_list::UniqueDependenciesList
};

pub fn generate_go_build_requirements(
    project_manipulator: &dyn ProjectManipulator,
    dependency_tree: Arc<Mutex<DependencyTreeNode>>,
) -> Result<UniqueDependenciesList> {
    let build_requirements: Vec<Project> = serde_json::from_str(
        &project_manipulator.run_shell(
            "go list -m -json all | jq -s '[.[] | {name: .Path, version: (.Version // \"\"), license: \"\", repository: \"\"}]'".to_string()
        )?
    )?;

    let all_dependencies: HashMap<String, Project> = dependency_tree
        .lock()
        .unwrap()
        .flatten()
        .dependencies
        .into_iter()
        .map(|dependency| (
            format!(
                "{}-{}",
                dependency.name,
                dependency.version,
            ),
            dependency
        ))
        .collect();

    let build_requirements: Vec<Project> = build_requirements
        .iter()
        .filter_map(|requirement| {
            let key: String = format!("{}-{}", requirement.name, requirement.version);
            if let Some(dependency) = all_dependencies.get(&key) {
                Some(
                    Project::new(
                        requirement.name.clone(),
                        requirement.version.clone(),
                        dependency.license.clone(),
                        dependency.repository.clone(),
                        dependency.subdirectory.clone(),
                        dependency.checkout.clone(),
                    )
                )
            }
            else {
                None
            }
        })
        .collect();

    Ok(UniqueDependenciesList::new(build_requirements))
}
