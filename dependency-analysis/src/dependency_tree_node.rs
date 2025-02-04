use ptree::{write_tree, TreeBuilder};
use source_wand_common::project::Project;

#[derive(Debug, Clone)]
pub struct DependencyTreeNode {
    pub project: Project,
    pub dependencies: Vec<Box<DependencyTreeNode>>,
}

impl DependencyTreeNode {
    pub fn new(project: Project, dependencies: Vec<Box<DependencyTreeNode>>) -> Self {
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
                dependency.build_tree(tree);
            }
            tree.end_child();
        }
    }
}
