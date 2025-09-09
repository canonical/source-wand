use std::collections::{HashSet};
use std::fmt::{self};

// ### DependencyTreeNodeGo ###
use std::sync::{Arc,Mutex};
use serde::{Deserialize, Serialize};


use dashmap::DashMap;

impl<DependencyTreeNode: fmt::Debug> fmt::Debug for Graph<DependencyTreeNode> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Graph").field("nodes", &self.nodes).finish()
    }
}



// ### Graph Structure ###
/**
 * Graph is a hashmap of nodes
 * Key: Node Name
 * Value: 
 */
#[derive(Clone)]
pub struct Graph<T> {
    pub nodes: Arc<DashMap<String, T>>,
    pub edges: Arc<DashMap<String, HashSet<String>>>,
}

impl<T: Clone> Graph<T> {
    pub fn new() -> Self {
        Graph {
            nodes: Arc::new(DashMap::new()),
            edges: Arc::new(DashMap::new()), // (Key: Parent Node Name & Version) | (Value: HashSet (Strings to Nodes))
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

    pub fn get_node_list(&self) -> Vec<String> {
        self.nodes.iter().map(|entry| entry.key().clone()).collect()
    }

    pub fn get_node(&self, key: &str) -> Option<T> {
        self.nodes.get(key).map(|node_ref| node_ref.value().clone())
    }

    pub fn get_edges(&self, key: &str) -> Option<HashSet<String>> {
        self.edges.get(key).map(|edge_ref| edge_ref.value().clone())
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
    pub fn print_graph(&self) {
        println!("Graph Contents:");

        // Get all node keys from the nodes map to ensure every node is included.
        let mut sorted_node_keys: Vec<String> = self.nodes.iter().map(|entry| entry.key().clone()).collect();
        sorted_node_keys.sort();

        // Iterate over every node key.
        for key in sorted_node_keys {
            // Check if the node has any outgoing edges.
            if let Some(edges_set) = self.edges.get(&key) {
                // If it has edges, sort them for consistent printing.
                let mut sorted_edges: Vec<&String> = edges_set.value().iter().collect();
                sorted_edges.sort();
                println!("{}: {:?}", key, sorted_edges);
            } else {
                // If the node has no outgoing edges, print an empty list.
                println!("{}: []", key);
            }
        }
    }
}


