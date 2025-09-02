use std::collections::{HashSet};
use std::fmt::{self};

// ### DependencyTreeNodeGo ###
use std::sync::{Arc,Mutex};
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone)]
pub struct DependencyTreeNodeGo {
    pub project: Project,
    pub dependencies: Vec<Arc<Mutex<DependencyTreeNodeGo>>>,
    pub rdependencies: Vec<Arc<Mutex<DependencyTreeNodeGo>>>,
}

impl DependencyTreeNodeGo {
    pub fn new_with_deps(project: Project, 
        dependencies: Vec<Arc<Mutex<DependencyTreeNodeGo>>>, 
        rdependencies: Vec<Arc<Mutex<DependencyTreeNodeGo>>>,) -> Self {
        DependencyTreeNodeGo {
            project,
            dependencies,
            rdependencies,
        }
    }
    
    pub fn new(project: Project) -> Self {
        let dependencies: Vec<Arc<Mutex<DependencyTreeNodeGo>>> = Vec::new();
        let rdependencies: Vec<Arc<Mutex<DependencyTreeNodeGo>>> = Vec::new();
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
use dashmap::DashMap;
use source_wand_common::project::Project;

#[derive(Clone)]
pub struct Graph<T> {
    pub nodes: Arc<DashMap<String, T>>,
    pub edges: Arc<DashMap<String, HashSet<String>>>,
}

impl<T> Graph<T> {
    pub fn new() -> Self {
        Graph {
            nodes: Arc::new(DashMap::new()),
            edges: Arc::new(DashMap::new()),
        }
    }

    pub fn does_key_exist(&self, key: &str) -> bool {
        self.nodes.contains_key(key)
    }

    pub fn add_node(&self, key: String, node: T) {
        self.nodes.insert(key, node);
    }

    pub fn add_depends(&self, parent: &str, child: &str) {
        self.edges.entry(parent.to_string())
            .or_insert_with(HashSet::new)
            .insert(child.to_string());
    }

    pub fn print_dependencies(&self) {
        println!("Project Dependencies:");

        let mut sorted_keys: Vec<String> = self.edges.iter().map(|entry| entry.key().clone()).collect();
        sorted_keys.sort();

        for key in sorted_keys {
            if let Some(deps) = self.edges.get(&key) {
                let mut dep_keys: Vec<&String> = deps.value().iter().collect();
                dep_keys.sort();
                println!("\"{}\": {:?}", key, dep_keys);
            } else {
                println!("\"{}\": []", key);
            }
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


//    pub fn to_dot(&self) -> String {
//        let mut dot_string = String::from("digraph DependencyGraph {\n");
//        
//        // --- Graph-level styling ---
//        dot_string.push_str("    rankdir=TB;\n"); // Top-to-Bottom layout
//        dot_string.push_str("    node [shape=box, style=\"rounded,filled\", fillcolor=lightgrey];\n");
//        dot_string.push_str("    edge [color=gray40];\n\n");
//
//        // Use a HashSet to avoid defining the same node or edge multiple times.
//        let mut defined_nodes = HashSet::new();
//
//        // Iterate through all nodes in the graph to define them and their edges.
//        for node in self.nodes.values() {
//            let node_name = &node.project.name;
//
//            // Define the current node if it hasn't been defined yet.
//            // The label can contain more info, like the checkout hash.
//            if defined_nodes.insert(node_name.clone()) {
//                let label = format!("{}\\n({})", node_name, node.project.checkout);
//                dot_string.push_str(&format!("    \"{}\" [label=\"{}\"];\n", node_name, label));
//            }
//
//            // Iterate through the node's dependencies to create edges.
//            for dependency in node.dependencies.borrow().iter() {
//                let dep_name = &dependency.project.name;
//
//                // Define the dependency node if it's not already part of the main node list
//                // or hasn't been seen yet.
//                if defined_nodes.insert(dep_name.clone()) {
//                    let label = format!("{}\\n({})", dep_name, dependency.project.checkout);
//                    dot_string.push_str(&format!("    \"{}\" [label=\"{}\"];\n", dep_name, label));
//                }
//
//                // Add the directed edge from the current node to its dependency.
//                dot_string.push_str(&format!("    \"{}\" -> \"{}\";\n", node_name, dep_name));
//            }
//        }
//
//        dot_string.push_str("}\n");
//        dot_string
//    }

impl<DependencyTreeNodeGo: fmt::Debug> fmt::Debug for Graph<DependencyTreeNodeGo> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Graph").field("nodes", &self.nodes).finish()
    }
}

