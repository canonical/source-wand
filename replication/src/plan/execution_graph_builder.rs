use std::{cell::RefCell, rc::Rc, sync::Arc};

use crate::plan::{transformation::Transformation, transformation_node::TransformationNode};

pub type RcExecutionNodeBuilder = Rc<RefCell<ExecutionNodeBuilder>>;

pub struct ExecutionGraphBuilder {
    pub node_builders: Vec<RcExecutionNodeBuilder>,
}

impl ExecutionGraphBuilder {
    pub fn new() -> Self {
        ExecutionGraphBuilder { node_builders: Vec::new() }
    }

    pub fn create_node(&mut self, workdesk: String, transformation: Arc<dyn Transformation>) -> RcExecutionNodeBuilder {
        let node: RcExecutionNodeBuilder = Rc::new(RefCell::new(ExecutionNodeBuilder::new(workdesk, transformation)));
        self.node_builders.push(node.clone());
        node
    }

    pub fn build(&self) -> Vec<Arc<TransformationNode>> {
        self.node_builders.iter().map(|node| Arc::new(node.borrow().build())).collect()
    }
}

pub struct ExecutionNodeBuilder {
    pub node: TransformationNode,
}

impl ExecutionNodeBuilder {
    pub fn new(workdesk: String, transformation: Arc<dyn Transformation>) -> Self {
        let node: TransformationNode = TransformationNode::new(
            workdesk,
            transformation,
            Vec::new(),
            Vec::new(),
        );
        ExecutionNodeBuilder { node }
    }

    pub fn depends_on(&mut self, other: &mut RcExecutionNodeBuilder) {
        self.node.dependencies.push(other.borrow().node.id);
        other.borrow_mut().node.dependents.push(self.node.id);
    }

    pub fn build(&self) -> TransformationNode {
        self.node.clone()
    }
}
