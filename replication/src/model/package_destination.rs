use serde::{Serialize, Deserialize};

use crate::model::package_destination_git::PackageDestinationGit;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PackageDestination {
    Git(PackageDestinationGit),
}
