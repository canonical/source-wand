use std::collections::{HashMap, HashSet};
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
    pub rdependencies: RefCell<Vec<Rc<DependencyTreeNodeGo>>>,
}

impl DependencyTreeNodeGo {
    pub fn new_with_deps(project: GoProject, 
        dependencies: RefCell<Vec<Rc<DependencyTreeNodeGo>>>, 
        rdependencies: RefCell<Vec<Rc<DependencyTreeNodeGo>>>,) -> Self {
        DependencyTreeNodeGo {
            project,
            dependencies,
            rdependencies,
        }
    }
    
    pub fn new(project: GoProject) -> Self {
        let dependency_nodes: Vec<Rc<DependencyTreeNodeGo>> = Vec::new();
        let rdependency_nodes: Vec<Rc<DependencyTreeNodeGo>> = Vec::new();
        let dependencies = RefCell::new(dependency_nodes);
        let rdependencies= RefCell::new(rdependency_nodes); 
        DependencyTreeNodeGo {
            project,
            dependencies,
            rdependencies
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

    pub fn add_rdepends(&mut self, parent_key: &String, child_key: &String) {
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

//    pub fn print_nodes(&self) {
//        let mut keys_by_address: HashMap<*const DependencyTreeNodeGo, &String> = HashMap::new();
//        for (key, node) in &self.nodes {
//            keys_by_address.insert(Rc::as_ptr(node), key);
//        }
//        for (key, node) in &self.nodes {
//            print!("Node: {}", key);
//            let dependencies = node.dependencies.borrow();
//
//
//
//        }
//    }


    pub fn to_dot(&self) -> String {
        let mut dot_string = String::from("digraph DependencyGraph {\n");
        
        // --- Graph-level styling ---
        dot_string.push_str("    rankdir=TB;\n"); // Top-to-Bottom layout
        dot_string.push_str("    node [shape=box, style=\"rounded,filled\", fillcolor=lightgrey];\n");
        dot_string.push_str("    edge [color=gray40];\n\n");

        // Use a HashSet to avoid defining the same node or edge multiple times.
        let mut defined_nodes = HashSet::new();

        // Iterate through all nodes in the graph to define them and their edges.
        for node in self.nodes.values() {
            let node_name = &node.project.name;

            // Define the current node if it hasn't been defined yet.
            // The label can contain more info, like the checkout hash.
            if defined_nodes.insert(node_name.clone()) {
                let label = format!("{}\\n({})", node_name, node.project.checkout);
                dot_string.push_str(&format!("    \"{}\" [label=\"{}\"];\n", node_name, label));
            }

            // Iterate through the node's dependencies to create edges.
            for dependency in node.dependencies.borrow().iter() {
                let dep_name = &dependency.project.name;

                // Define the dependency node if it's not already part of the main node list
                // or hasn't been seen yet.
                if defined_nodes.insert(dep_name.clone()) {
                    let label = format!("{}\\n({})", dep_name, dependency.project.checkout);
                    dot_string.push_str(&format!("    \"{}\" [label=\"{}\"];\n", dep_name, label));
                }

                // Add the directed edge from the current node to its dependency.
                dot_string.push_str(&format!("    \"{}\" -> \"{}\";\n", node_name, dep_name));
            }
        }

        dot_string.push_str("}\n");
        dot_string
    }


}

////////////////////////////////////////////////////////////////////////////////////////////////////
// ### Project ###
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoProject {
    pub name: String, //module name
    pub version: String,
    //pub license: String,
    pub repository_url: String,
    //pub subdirectory: Option<String>,
    pub checkout: String,
    // pub sourcecraft_track: Option<String>,
    // pub sourcecraft_name: String
    // pub sourcecraft_risk
}

impl GoProject {
    pub fn new(
        name: String,
        version: String,
        //license: String,
        repository_url: String,
        //subdirectory: Option<String>,
        checkout: String,
    ) -> Self {
        GoProject {
            name,
            version,
            //license,
            repository_url,
            //subdirectory,
            checkout,
        }
    }
}

