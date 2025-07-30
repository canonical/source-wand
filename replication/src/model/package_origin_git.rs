use serde::{Serialize, Deserialize};

use crate::model::package_origin::PackageOrigin;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageOriginGit {
    pub git: String,
    pub reference: String, 
}

impl PackageOriginGit {
    pub fn new(git: String, reference: String) -> PackageOrigin {
        PackageOrigin::Git(PackageOriginGit { git, reference })
    }
}
