use std::collections::HashMap;

use serde::Serialize;

use crate::model::sourcecraft::part::Part;

#[derive(Debug, Clone, Serialize)]
pub struct SourcecraftMetadata {
    pub name: String,
    pub version: String,
    pub base: String,
    pub summary: String,
    pub description: String,
    pub platforms: HashMap<String, ()>,
    pub parts: HashMap<String, Part>
}
