use regex::Regex;

#[readonly::make]
pub struct Environment {
    pub name: String,
    pub version: String,
    pub version_major: String,
    pub version_minor: String,
    pub version_patch: String,
    pub version_suffix: String,
    pub version_retrocompatible: String,
}

impl Environment {
    pub fn new(name: &String, version: &String) -> Self {
        let mut major: String = String::new();
        let mut minor: String = String::new();
        let mut patch: String = String::new();
        let mut suffix: String = String::new();

        if version.starts_with('v') {
            let parts: Vec<&str> = version.trim_start_matches('v').split('-').collect();
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

        let mut formatted_name: String;

        if name.starts_with("go-") {
            formatted_name = name.clone();
        } else {
            formatted_name = format!("go-{}", name);
        }

        if formatted_name.len() > 40 {
            let parts: Vec<&str> = formatted_name.split('-').collect();

            if parts.len() > 2 {
                let domain_prefix = format!("{}-{}", parts[0], parts[1]);
                let mut new_parts: Vec<&str> = vec![];

                for i in (2..parts.len()).rev() {
                    let mut accumulative_name: String = String::new();

                    accumulative_name.push_str(&domain_prefix);
                    accumulative_name.push('-');
                    accumulative_name.push_str(&new_parts.iter().rev().cloned().collect::<Vec<&str>>().join("-"));
                    accumulative_name.push('-');
                    accumulative_name.push_str(parts[i]);

                    if accumulative_name.len() < 40 {
                        new_parts.push(parts[i]);
                    } else {
                        break;
                    }
                }

                new_parts.reverse();
                formatted_name = format!("{}-{}", domain_prefix, new_parts.join("-"));
            }
            else {
                let max_length: usize = 40;
                let prefix_length: usize = 3;

                let start_index: usize = formatted_name.len() - (max_length - prefix_length);
                let new_suffix: &str = &formatted_name[start_index..];

                formatted_name = format!("go-{}", new_suffix);
            }
        }

        Environment {
            name: formatted_name,
            version: version.clone(),
            version_major: major,
            version_minor: minor,
            version_patch: patch,
            version_suffix: suffix,
            version_retrocompatible: retrocompatible,
        }
    }

    pub fn apply(&self, template: &String) -> String {
        template
            .replace("$NAME", &self.name)
            .replace("$VERSION_MAJOR", &self.version_major)
            .replace("$VERSION_MINOR", &self.version_minor)
            .replace("$VERSION_PATCH", &self.version_patch)
            .replace("$VERSION_SUFFIX", &self.version_suffix)
            .replace("$VERSION_RETROCOMPATIBLE", &self.version_retrocompatible)
            .replace("$VERSION", &self.version)
    }
}
