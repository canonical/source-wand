use std::collections::{HashMap, HashSet};
use std::fmt::{self, Display};
// ### DependencyTreeNodeGo ###
use std::sync::{Arc,Mutex};
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone)]
pub struct DependencyTreeNodeGo {
    pub project: GoProject,
    pub dependencies: Vec<Arc<Mutex<DependencyTreeNodeGo>>>,
    pub rdependencies: Vec<Arc<Mutex<DependencyTreeNodeGo>>>,
}

impl DependencyTreeNodeGo {
    pub fn new_with_deps(project: GoProject, 
        dependencies: Vec<Arc<Mutex<DependencyTreeNodeGo>>>, 
        rdependencies: Vec<Arc<Mutex<DependencyTreeNodeGo>>>,) -> Self {
        DependencyTreeNodeGo {
            project,
            dependencies,
            rdependencies,
        }
    }
    
    pub fn new(project: GoProject) -> Self {
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
pub struct Graph<DependencyTreeNodeGo, String> {
    pub nodes: HashMap<String, Arc<Mutex<DependencyTreeNodeGo>>>,
}

impl Graph<DependencyTreeNodeGo, String> {
    pub fn new() -> Self {
        Graph { nodes: HashMap::new()}
    }

    pub fn add_node(&mut self, key: String, data: DependencyTreeNodeGo) -> Arc<Mutex<DependencyTreeNodeGo>> {
        let node_handle = Arc::new(Mutex::new(data));
        self.nodes.insert(key, node_handle.clone());
        node_handle
    }

    fn add_edge(&self, from: &Arc<Mutex<DependencyTreeNodeGo>>, to: &Arc<Mutex<DependencyTreeNodeGo>>) {
        let mut from_node = from.lock().unwrap();
        from_node.dependencies.push(to.clone())
    }

    /// Adds an edge (to_key) into (from_key)'s `dependencies` list
    pub fn add_depends(&mut self, from_key: &String, to_key: &String) -> bool {
        if let (Some(from_node), Some(to_node)) = 
        (self.nodes.get(from_key), self.nodes.get(to_key)) {
            self.add_edge(from_node, to_node);
            true
        } else {
            false
        }
    }

    pub fn does_key_exist(&mut self, key: &String) -> bool {
        self.nodes.contains_key(key)
    }

    pub fn print_dependencies(&self) {
        println!("Project Dependencies:");

        // Create a reverse map from a node's memory address to its key.
        let mut keys_by_address: HashMap<*const Mutex<DependencyTreeNodeGo>, &String> = HashMap::new();
        for (key, node_handle) in &self.nodes {
            keys_by_address.insert(Arc::as_ptr(node_handle), key);
        }

        // Get and sort keys for a consistent, ordered output.
        let mut sorted_keys: Vec<&String> = self.nodes.keys().collect();
        sorted_keys.sort();

        // Iterate and print each node's dependencies.
        for key in sorted_keys {
            let node_handle = self.nodes.get(key).unwrap();
            let node = node_handle.lock().unwrap();
            
            let dependency_keys: Vec<&String> = node.dependencies.iter()
                .filter_map(|dep_handle| keys_by_address.get(&Arc::as_ptr(dep_handle)).copied())
                .collect();
            
            println!("\"{}\": {:?}", key, dependency_keys);
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


}

impl<DependencyTreeNodeGo: fmt::Debug, String: fmt::Debug> fmt::Debug for Graph<DependencyTreeNodeGo, String> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Graph").field("nodes", &self.nodes).finish()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// ### Project ###

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

