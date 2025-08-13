use serde::{Serialize, Deserialize};

use crate::model::{package::Package, replication_project::ReplicationProject};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationPlan {
    pub project: ReplicationProject,
    pub packages: Vec<Package>,
}

impl ReplicationPlan {
    pub fn new(project: ReplicationProject, packages: Vec<Package>) -> Self {
        ReplicationPlan { project, packages }
    }
}
