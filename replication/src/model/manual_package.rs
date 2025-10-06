use serde::{Serialize, Deserialize};

use crate::model::{package_destination::PackageDestination, package_origin::PackageOrigin};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManualPackage {
    pub origin: PackageOrigin,
    pub destination: PackageDestination,

    pub track: Option<String>,
}

impl ManualPackage {
    pub fn new(
        origin: PackageOrigin,
        destination: PackageDestination,
        track: Option<String>,
    ) -> Self {
        ManualPackage { origin, destination, track }
    }
}
