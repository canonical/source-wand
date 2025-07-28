use serde::{Serialize, Deserialize};

use crate::model::{dependency::Dependency, package_destination::PackageDestination, package_origin::PackageOrigin};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub level: u32,
    pub origin: PackageOrigin,
    pub destination: PackageDestination,
    pub dependencies: Vec<Dependency>,
}
