use serde::{Serialize, Deserialize};

use crate::model::package_origin::PackageOrigin;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageOriginGoCache {
    pub name: String,
    pub version: String,
    pub path: String,
}

impl PackageOriginGoCache {
    pub fn new(name: String, version: String, path: String) -> PackageOrigin {
        PackageOrigin::GoCache(PackageOriginGoCache { name, version, path })
    }
}
