use dashmap::DashSet;

use crate::transformation_node::NodeId;

pub struct ExecutionProgressTracker {
    executing: DashSet<NodeId>,
    completed: DashSet<NodeId>,
}

impl ExecutionProgressTracker {
    pub fn new() -> Self {
        ExecutionProgressTracker {
            executing: DashSet::new(),
            completed: DashSet::new(),
        }
    }

    pub fn reserve(&self, node: NodeId) {
        self.executing.insert(node);
    }

    pub fn complete(&self, node: NodeId) {
        self.executing.remove(&node);
        self.completed.insert(node);
    }

    pub fn is_available(&self, node: &NodeId) -> bool {
        self.executing.get(node).is_none() && self.completed.get(node).is_none()
    }

    pub fn has_completed(&self, node: &NodeId) -> bool {
        self.completed.contains(node)
    }

    pub fn count_completed(&self) -> usize {
        self.completed.len()
    }
}
