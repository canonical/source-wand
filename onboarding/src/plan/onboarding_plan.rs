use serde::{Deserialize, Serialize};

use super::onboarding_source::OnboardingSource;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnboardingPlan {
    pub name: String,
    pub version: String,
    pub license: String,

    pub source: OnboardingSource,

    pub sourcecraft_track: String,
    pub depends_on: Vec<String>,
}

impl OnboardingPlan {
    pub fn new(
        name: String,
        version: String,
        license: String,
        source: OnboardingSource,
        sourcecraft_track: String,
        depends_on: Vec<String>,
    ) -> Self {
        OnboardingPlan {
            name,
            version,
            license,
            source,
            sourcecraft_track,
            depends_on
        }
    }
}
