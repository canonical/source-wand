use std::sync::Arc;

use crate::plan::transformation::Transformation;

pub type NodeId = usize;

#[derive(Clone)]
pub struct TransformationNode {
    pub id: NodeId,
    pub workdesk: String,
    pub transformation: Arc<dyn Transformation>,
    pub dependencies: Vec<NodeId>,
}
