use std::sync::{Arc, Mutex};

use ptree::{write_tree, TreeBuilder};
use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};

use source_wand_common::project::Project;

#[derive(Debug, Clone)]
pub struct DependencyTreeNode {
    pub project: Project,
    pub dependencies: Vec<Arc<Mutex<DependencyTreeNode>>>,
}

impl DependencyTreeNode {
    pub fn new(project: Project, dependencies: Vec<Arc<Mutex<DependencyTreeNode>>>) -> Self {
        DependencyTreeNode {
            project,
            dependencies,
        }
    }

    pub fn to_string(&self) -> Result<String, String> {
        let mut buffer: Vec<u8> = Vec::new();
        let mut tree: TreeBuilder = TreeBuilder::new("(Dependencies)".to_string());
        self.build_tree(&mut tree);

        match write_tree(&tree.build(), &mut buffer) {
            Ok(_) => {
                match String::from_utf8(buffer) {
                    Ok(s) => Ok(s),
                    Err(e) => Err(format!("Failed to convert buffer to string: {}", e)),
                }
            },
            Err(e) => Err(format!("Failed to write tree: {}", e)),
        }
    }

    fn build_tree(&self, tree: &mut TreeBuilder) {
        let text = format!("{} {}", self.project.name, self.project.version);
        if self.dependencies.is_empty() {
            tree.add_empty_child(text);
        }
        else {
            tree.begin_child(text);

            for dependency in &self.dependencies {
                dependency.lock().unwrap().build_tree(tree);
            }

            tree.end_child();
        }
    }
}

impl Serialize for DependencyTreeNode {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state: <S as Serializer>::SerializeStruct = serializer.serialize_struct("DependencyTreeNode", 2)?;
        state.serialize_field("project", &self.project)?;

        let dependencies: Vec<DependencyTreeNode> = self.dependencies
            .iter()
            .map(|dependency| dependency.lock().unwrap().to_owned())
            .collect();

        state.serialize_field("dependencies", &dependencies)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for DependencyTreeNode {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct Helper {
            project: Project,
            dependencies: Vec<DependencyTreeNode>,
        }

        let Helper { project, dependencies } = Helper::deserialize(deserializer)?;

        let dependencies: Vec<Arc<Mutex<DependencyTreeNode>>> = dependencies
            .into_iter()
            .map(|dep| Arc::new(Mutex::new(dep)))
            .collect();

        Ok(DependencyTreeNode { project, dependencies })
    }
}
