use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageOriginGit {
    pub git: String,
    pub reference: String, 
}
