use regex::Regex;

#[readonly::make]
pub struct SemanticVersion {
    pub raw: String,
    pub major: String,
    pub minor: String,
    pub patch: String,
    pub suffix: String,
    pub retrocompatible: String,
}

impl SemanticVersion {
    pub fn new(original_version: &String) -> Self {
        let mut major: String = String::new();
        let mut minor: String = String::new();
        let mut patch: String = String::new();
        let mut suffix: String = String::new();

        if original_version.starts_with('v') {
            let parts: Vec<&str> = original_version.trim_start_matches('v').split('-').collect();
            let semantic_version_parts: Vec<&str> = parts[0].split('.').collect();

            if semantic_version_parts.len() > 0 {
                major = semantic_version_parts[0].to_string();
            }
            if semantic_version_parts.len() > 1 {
                minor = semantic_version_parts[1].to_string();
            }
            if semantic_version_parts.len() > 2 {
                patch = semantic_version_parts[2].to_string();
            }

            if parts.len() > 1 {
                suffix = format!("-{}", parts[1..].join("-"));
            }
        }

        let retrocompatible = if !suffix.is_empty() {
            let re: Regex = Regex::new(r"(\d{14})-([a-f0-9]{12,40})$").unwrap();
            if let Some(caps) = re.captures(&suffix) {
                let datetime_str: &str = caps.get(1).unwrap().as_str();
                let hash: &str = caps.get(2).unwrap().as_str();

                let year: &str = &datetime_str[0..4];
                let month: &str = &datetime_str[4..6];
                let day: &str = &datetime_str[6..8];

                format!("{}{}{}-{}", year, month, day, &hash[0..7])
            } else {
                format!("{}.{}.{}-{}", major, minor, patch, suffix)
            }
        } else {
            if major == "0".to_string() {
                format!("{}.{}.{}", major.clone(), minor, patch)
            } else {
                major.clone()
            }
        };

        SemanticVersion {
            raw: original_version.clone(),
            major,
            minor,
            patch,
            suffix,
            retrocompatible,
        }
    }

    pub fn apply(&self, template: &String) -> String {
        template
            .replace("$VERSION_MAJOR", &self.major)
            .replace("$VERSION_MINOR", &self.minor)
            .replace("$VERSION_PATCH", &self.patch)
            .replace("$VERSION_SUFFIX", &self.suffix)
            .replace("$VERSION_RETROCOMPATIBLE", &self.retrocompatible)
            .replace("$VERSION", &self.raw)
    }
}
