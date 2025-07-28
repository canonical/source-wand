use serde::{Serialize, Deserialize};

use crate::model::package_origin_git::PackageOriginGit;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PackageOrigin {
    Git(PackageOriginGit),
}
