use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hooks {
    pub before_all: Option<String>,

    pub before_each: Option<String>,
    pub after_each: Option<String>,

    pub after_all: Option<String>,
}
