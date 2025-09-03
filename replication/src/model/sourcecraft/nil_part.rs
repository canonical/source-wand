use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct NilPart {
    pub plugin: String,
    pub source: String,
}
