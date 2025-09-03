use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationConfig {
    pub git_identity: Option<GitIdentity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitIdentity {
    pub username: String,
    pub email: String,
}
