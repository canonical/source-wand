use serde::{Serialize, Deserialize};

use crate::model::{dependency::Dependency, package_destination::PackageDestination, package_origin::PackageOrigin};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub origin: PackageOrigin,
    pub destination: PackageDestination,
    pub dependencies: Vec<Dependency>,
    pub is_library: bool,
}

impl Package {
    pub fn new(
        origin: PackageOrigin,
        destination: PackageDestination,
        dependencies: Vec<Dependency>,
        is_library: bool,
    ) -> Self {
        Package { origin, destination, dependencies, is_library }
    }
}
