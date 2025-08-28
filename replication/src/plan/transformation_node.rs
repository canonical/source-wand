use std::sync::Arc;

use uuid::Uuid;

use crate::plan::transformation::Transformation;

pub type NodeId = Uuid;

#[derive(Clone)]
pub struct TransformationNode {
    pub id: NodeId,
    pub workdesk: String,
    pub transformation: Arc<dyn Transformation>,
    pub dependencies: Vec<NodeId>,
}

impl TransformationNode {
    pub fn new(workdesk: String, transformation: Arc<dyn Transformation>, dependencies: Vec<NodeId>) -> Self {
        let id: NodeId = Uuid::new_v4();
        TransformationNode { id, workdesk, transformation, dependencies }
    }
}
