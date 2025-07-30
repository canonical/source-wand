use serde::{Serialize, Deserialize};

use crate::model::{hooks::Hooks, package_destination::PackageDestination, package_origin::PackageOrigin};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationManifest {
    pub project: String,
    pub hooks: Option<Hooks>,
    pub origin: PackageOrigin,
    pub destination_template: PackageDestination,
}

impl ReplicationManifest {
    pub fn new(project: String, hooks: Option<Hooks>, origin: PackageOrigin, destination_template: PackageDestination) -> Self {
        ReplicationManifest { project, hooks, origin, destination_template }
    }
}
