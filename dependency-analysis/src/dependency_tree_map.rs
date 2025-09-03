use std::collections::{HashMap, HashSet};

use source_wand_common::project::Project;

use crate::dependency_tree_node::DependencyTreeNode;

pub type DependencyTreeMap = HashMap<String, Vec<Project>>;

impl DependencyTreeNode {
    pub fn to_map(&self) -> DependencyTreeMap {
        let mut map: DependencyTreeMap = DependencyTreeMap::new();
        let mut visited: HashSet<String> = HashSet::new();

        self.traverse_for_map(&mut map, &mut visited);

        map
    }

    fn traverse_for_map(&self, map: &mut DependencyTreeMap, visited: &mut HashSet<String>) {
        let project_hash: String = format!("{}-{}", self.project.name, self.project.version);

        if visited.insert(project_hash.clone()) {
            let dependencies: Vec<Project> = self.dependencies
                .iter()
                .map(
                    |dependency| {
                        dependency.lock().unwrap().project.clone()
                    }
                )
                .collect();
            map.insert(project_hash, dependencies);
        }

        for dependency in &self.dependencies {
            dependency.lock().unwrap().traverse_for_map(map, visited);
        }
    }
}
