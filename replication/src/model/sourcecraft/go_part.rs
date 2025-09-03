use std::collections::HashMap;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct GoPart {
    pub plugin: String,
    pub source: String,
    pub build_snaps: Option<Vec<String>>,
    pub build_environment: Option<Vec<HashMap<String, String>>>,
    pub after: Option<Vec<String>>,
}
