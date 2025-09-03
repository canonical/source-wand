use serde::{Serialize, Deserialize};

use crate::model::{hooks::Hooks, package_destination::PackageDestination, package_origin::PackageOrigin, replication_config::ReplicationConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationManifest {
    pub project: String,
    pub hooks: Option<Hooks>,
    pub origin: PackageOrigin,
    pub destination_template: PackageDestination,

    pub config: Option<ReplicationConfig>,
}

impl ReplicationManifest {
    pub fn new(
        project: String,
        hooks: Option<Hooks>,
        origin: PackageOrigin,
        destination_template: PackageDestination,
        config: Option<ReplicationConfig>,
    ) -> Self {
        ReplicationManifest { project, hooks, origin, destination_template, config }
    }
}
