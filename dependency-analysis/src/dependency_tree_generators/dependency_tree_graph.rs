use std::collections::{HashSet};
use std::fmt::{self};

// ### DependencyTreeNodeGo ###
use std::sync::{Arc,Mutex, MutexGuard};
use serde::{Deserialize, Serialize};


use dashmap::DashMap;

use crate::dependency_tree_node::DependencyTreeNode;

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

impl Graph<DependencyTreeNode> {
    pub fn to_dependency_tree_node(&self) -> Option<Arc<Mutex<DependencyTreeNode>>> {
        let nodes: Arc<DashMap<String, DependencyTreeNode>> = self.nodes.clone();
        let edges: Arc<DashMap<String, HashSet<String>>> = self.edges.clone();

        let tree_nodes: DashMap<String, Arc<Mutex<DependencyTreeNode>>> = DashMap::new();

        for entry in nodes.iter() {
            let (key, value) = entry.pair();
            tree_nodes.insert(key.clone(), Arc::new(Mutex::new(value.clone())));
        }

        for entry in edges.iter() {
            let (parent_key, children_keys) = entry.pair();
            if let Some(parent_node_mutex) = tree_nodes.get(parent_key) {
                let mut parent_node: MutexGuard<'_, DependencyTreeNode> = parent_node_mutex.lock().unwrap();
                for child_key in children_keys.iter() {
                    if let Some(child_node_mutex) = tree_nodes.get(child_key) {
                        parent_node.dependencies.push(child_node_mutex.clone());
                    }
                }
            }
        }

        let mut all_children: HashSet<String> = HashSet::new();
        for children_keys in edges.iter().map(|entry| entry.value().clone()) {
            all_children.extend(children_keys.iter().cloned());
        }

        let root_node_key: Option<String> = edges.iter()
            .map(|entry| entry.key().clone())
            .find(|parent_key| !all_children.contains(parent_key));

        if let Some(root_key) = root_node_key {
            tree_nodes.get(&root_key).map(|entry| entry.value().clone())
        } else {
            None
        }
    }
}
