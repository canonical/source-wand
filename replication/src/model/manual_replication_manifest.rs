use serde::{Serialize, Deserialize};

use crate::model::{
    manual_package::ManualPackage,
    replication_config::ReplicationConfig,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManualReplicationManifest {
    pub packages: Vec<ManualPackage>,
    pub config: Option<ReplicationConfig>,
}

impl ManualReplicationManifest {
    pub fn new(
        packages: Vec<ManualPackage>,
        config: Option<ReplicationConfig>,
    ) -> Self {
        ManualReplicationManifest { packages, config }
    }
}
