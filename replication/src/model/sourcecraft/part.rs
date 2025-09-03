use std::collections::HashMap;

use serde::Serialize;

use crate::model::sourcecraft::{go_part::GoPart, go_use_part::GoUsePart, nil_part::NilPart};

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum Part {
    Nil(NilPart),
    Go(GoPart),
    GoUse(GoUsePart),
}

impl Part {
    pub fn with_nil_plugin() -> Self {
        Part::Nil(NilPart { plugin: "nil".to_string(), source: ".".to_string() })
    }

    pub fn with_go_plugin(
        source: String,
        build_snaps: Vec<String>,
        build_environment: Vec<HashMap<String, String>>,
        after: Vec<String>,
    ) -> Self {
        Part::Go(
            GoPart {
                plugin: "go".to_string(),
                source,
                build_snaps: Some(build_snaps),
                build_environment: Some(build_environment),
                after: Some(after),
            }
        )
    }

    pub fn with_go_use_plugin(
        name: String,
        track: String,
    ) -> Self {
        Part::GoUse(
            GoUsePart {
                plugin: "go-use".to_string(),
                source: format!("sourcecraft:{}", name),
                source_channel: Some(format!("{}/edge", track)),
            }
        )
    }
}
