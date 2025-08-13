use serde::{Deserialize, Serialize};

use crate::model::{package_destination::PackageDestination, package_origin::PackageOrigin};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationProject {
    pub origin: PackageOrigin,
    pub destination_template: PackageDestination,
}

impl ReplicationProject {
    pub fn new(origin: PackageOrigin, destination_template: PackageDestination) -> Self {
        ReplicationProject { origin, destination_template }
    }
}
