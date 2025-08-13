use serde::{Serialize, Deserialize};

use crate::model::replication_project::ReplicationProject;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationManifest {
    pub project: ReplicationProject,
}

impl ReplicationManifest {
    pub fn new(project: ReplicationProject) -> Self {
        ReplicationManifest { project }
    }
}
