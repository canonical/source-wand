use serde::{Serialize, Deserialize};

use crate::model::{hooks::Hooks, package::Package};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationPlan {
    pub project: String,
    pub hooks: Option<Hooks>,
    pub packages: Vec<Package>,
}

impl ReplicationPlan {
    pub fn new(project: String, hooks: Option<Hooks>, packages: Vec<Package>) -> Self {
        ReplicationPlan { project, hooks, packages }
    }
}
