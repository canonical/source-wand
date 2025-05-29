use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OnboardingSource {
    Git(OnboardingSourceGit),
    GitMonorepository(OnboardingSourceGitMonorepository),
    Sourcecraft(OnboardingSourceSourcecraft),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnboardingSourceGit {
    pub url: String,
    pub checkout: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnboardingSourceGitMonorepository {
    pub url: String,
    pub path: String,
    pub checkout: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnboardingSourceSourcecraft {
    pub sourcecraft: String,
    pub track: String,
}

impl OnboardingSource {
    pub fn git(url: String, checkout: String) -> Self {
        OnboardingSource::Git(OnboardingSourceGit { url, checkout })
    }

    pub fn git_monorepository(url: String, path: String, checkout: String) -> Self {
        OnboardingSource::GitMonorepository(OnboardingSourceGitMonorepository { url, path, checkout })
    }

    pub fn sourcecraft(sourcecraft: String, track: String) -> Self {
        OnboardingSource::Sourcecraft(OnboardingSourceSourcecraft { sourcecraft, track })
    }
}
