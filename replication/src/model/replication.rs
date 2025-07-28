use serde::{Serialize, Deserialize};

use crate::model::{hooks::Hooks, package::Package};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Replication {
    pub project: String,
    pub hooks: Hooks,
    pub packages: Vec<Package>,
}
