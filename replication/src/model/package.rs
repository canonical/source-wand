use serde::{Serialize, Deserialize};

use crate::model::{dependency::Dependency, package_destination::PackageDestination, package_origin::PackageOrigin};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub origin: PackageOrigin,
    pub destination: PackageDestination,
    pub dependencies: Vec<Dependency>,
}

impl Package {
    pub fn new(origin: PackageOrigin, destination: PackageDestination, dependencies: Vec<Dependency>) -> Self {
        Package { origin, destination, dependencies }
    }
}
