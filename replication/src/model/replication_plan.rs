use serde::{Serialize, Deserialize};

use crate::model::{hooks::Hooks, package::Package, replication_config::ReplicationConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationPlan {
    pub project: String,
    pub hooks: Option<Hooks>,
    pub packages: Vec<Package>,

    pub config: Option<ReplicationConfig>,
}

impl ReplicationPlan {
    pub fn new(
        project: String,
        hooks: Option<Hooks>,
        packages: Vec<Package>,
        config: Option<ReplicationConfig>,
    ) -> Self {
        ReplicationPlan { project, hooks, packages, config }
    }
}
