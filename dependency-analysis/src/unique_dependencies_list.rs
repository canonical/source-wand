use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use source_wand_common::project::Project;

use crate::dependency_tree_node::DependencyTreeNode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniqueDependenciesList {
    pub dependencies: Vec<Project>,
}

impl UniqueDependenciesList {
    pub fn new(dependencies: Vec<Project>) -> Self {
        UniqueDependenciesList { dependencies }
    }

    pub fn to_string(&self) -> String {
        let mut representation: String = "".to_string();

        for dependency in &self.dependencies {
            representation += format!("Project: {}\n", dependency.name).as_str();
            representation += format!("Version: {}\n", dependency.version).as_str();
            representation += format!("License: {}\n", dependency.license).as_str();
            representation += format!("Repository: {}\n", dependency.repository).as_str();
            representation += format!("\n").as_str();
        }

        representation
    }
}

impl DependencyTreeNode {
    pub fn flatten(&self) -> UniqueDependenciesList {
        let mut projects: Vec<Project> = Vec::new();
        let mut visited: HashSet<String> = HashSet::new();

        self.flatten_node(&mut projects, &mut visited);

        projects.sort_by(
            |a, b|
            format!("{}{}", a.name, a.version).cmp(
                &format!("{}{}", b.name, b.version)
            )
        );

        UniqueDependenciesList::new(projects)
    }

    fn flatten_node(&self, projects: &mut Vec<Project>, visited: &mut HashSet<String>) {
        let project_hash: String = format!("{}-{}", self.project.name, self.project.version);

        if visited.insert(project_hash) {
            projects.push(self.project.clone());
        }

        for dependency in &self.dependencies {
            dependency.flatten_node(projects, visited);
        }
    }
}
