use std::collections::HashSet;

use crate::transformation_node::NodeId;

pub struct ExecutionProgressTracker {
    executing: HashSet<NodeId>,
    completed: HashSet<NodeId>,
}

impl ExecutionProgressTracker {
    pub fn new() -> Self {
        ExecutionProgressTracker {
            executing: HashSet::new(),
            completed: HashSet::new(),
        }
    }

    pub fn reserve(&mut self, node: NodeId) {
        self.executing.insert(node);
    }

    pub fn complete(&mut self, node: NodeId) {
        self.executing.remove(&node);
        self.completed.insert(node);
    }

    pub fn is_available(&self, node: &NodeId) -> bool {
        !self.executing.contains(node) && !self.completed.contains(node)
    }

    pub fn has_completed(&self, node: &NodeId) -> bool {
        self.completed.contains(node)
    }

    pub fn count_completed(&self) -> usize {
        self.completed.len()
    }
}
