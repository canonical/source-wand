use std::collections::HashMap;
// ### DependencyTreeNodeGo ###
use std::rc::Rc;
use std::cell::RefCell;
use ptree::{write_tree, TreeBuilder};
/**
 * 
 */
pub struct DependencyTreeNodeGo {
    pub project: GoProject,
    pub dependencies: RefCell<Vec<Rc<DependencyTreeNodeGo>>>,
}

impl DependencyTreeNodeGo {
    pub fn new_with_deps(project: GoProject, dependencies: RefCell<Vec<Rc<DependencyTreeNodeGo>>>,) -> Self {
        DependencyTreeNodeGo {
            project,
            dependencies,
        }
    }
    
    pub fn new(project: GoProject) -> Self {
        let dependency_nodes: Vec<Rc<DependencyTreeNodeGo>> = Vec::new();
        let dependencies = RefCell::new(dependency_nodes);
        DependencyTreeNodeGo {
            project,
            dependencies,
        }
    }
}

// ### Graph Structure ###
/**
 * Graph is a hashmap of nodes
 * Key: Go Node Name
 * Value: Rc owners of DependencyTreeNodeGo
 */
pub struct Graph<DependencyTreeNodeGo, String> {
    pub nodes: HashMap<String, Rc<DependencyTreeNodeGo>>,
}

impl Graph<DependencyTreeNodeGo, String> {
    pub fn new() -> Self {
        Graph { nodes: HashMap::new()}
    }

    pub fn add_node(&mut self, key: String, data: DependencyTreeNodeGo) {
        let new_node = Rc::new(data);
        self.nodes.insert(key, new_node);
    }

    pub fn add_depends(&mut self, parent_key: &String, child_key: &String) {
        if let Some(parent_node) = self.nodes.get(parent_key) {
            if let Some(child_node) = self.nodes.get(child_key) {
                parent_node.dependencies.borrow_mut().push(Rc::clone(child_node));
            }
        }
    }
    pub fn does_key_exist(&mut self, key: &String) -> bool {
        self.nodes.contains_key(key)
    }
    pub fn print(&self) {
        println!("Graph Nodes and Dependencies:");
        let mut keys_by_address: HashMap<*const DependencyTreeNodeGo, &String> = HashMap::new();
        for (key, node) in &self.nodes {
            keys_by_address.insert(Rc::as_ptr(node), key);
        }

        for (key, node) in &self.nodes {
            print!("Node: {}", key);
            let dependencies = node.dependencies.borrow();
            if !dependencies.is_empty() {
                print!(" -> Dependencies: ");
                let dependency_keys: Vec<&String> = dependencies.iter()
                    .filter_map(|dep| {
                        let ptr = Rc::as_ptr(dep);
                        keys_by_address.get(&ptr).copied()
                    })
                    .collect();
                println!("{:?}", dependency_keys);
            } else {
                println!();
            }
        }
}
}


// ### Project ###
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoProject {
    pub name: String,
    //pub version: String,
    //pub license: String,
    pub repository_url: String,
    //pub subdirectory: Option<String>,
    pub checkout: String,
    // pub track: Option<String>,
}

impl GoProject {
    pub fn new(
        name: String,
        //version: String,
        //license: String,
        repository_url: String,
        //subdirectory: Option<String>,
        checkout: String,
    ) -> Self {
        GoProject {
            name,
            //version,
            //license,
            repository_url,
            //subdirectory,
            checkout,
        }
    }
}

