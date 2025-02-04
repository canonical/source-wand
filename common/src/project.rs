use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub version: String,
    pub license: String,
    pub repository: String,
}

impl Project {
    pub fn new(name: String, version: String, license: String, repository: String) -> Self {
        Project { name, version, license, repository }
    }
}
