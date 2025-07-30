use serde::{Serialize, Deserialize};

use crate::model::package_destination::PackageDestination;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageDestinationGit {
    pub git: String,
    pub reference: String,
}

impl PackageDestinationGit {
    pub fn new(git: String, reference: String) -> PackageDestination {
        PackageDestination::Git(PackageDestinationGit { git, reference })
    }
}
