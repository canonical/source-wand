use std::sync::Arc;

use uuid::Uuid;

use crate::transformation::Transformation;

pub type NodeId = Uuid;

#[derive(Clone)]
pub struct TransformationNode {
    pub id: NodeId,
    pub workdesk: String,
    pub transformation: Arc<dyn Transformation>,
    pub dependencies: Vec<NodeId>,
    pub dependents: Vec<NodeId>,
}

impl TransformationNode {
    pub fn new(
        workdesk: String,
        transformation: Arc<dyn Transformation>,
        dependencies: Vec<NodeId>,
        dependents: Vec<NodeId>
    ) -> Self {
        let id: NodeId = Uuid::new_v4();
        TransformationNode {
            id,
            workdesk,
            transformation,
            dependencies,
            dependents
        }
    }
}
