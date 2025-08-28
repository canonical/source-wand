use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct GoUsePart {
    pub plugin: String,
    pub source: String,
    pub source_channel: Option<String>,
}
