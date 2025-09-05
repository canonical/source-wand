#[readonly::make]
pub struct SanitizedName {
    pub original: String,
    pub sanitized: String,
}

impl SanitizedName {
    pub fn new(original_name: &String) -> Self {
        let mut sanitized_name: String;

        if original_name.starts_with("go-") {
            sanitized_name = original_name.clone();
        } else {
            sanitized_name = format!("go-{}", original_name);
        }

        if sanitized_name.len() > 40 {
            let parts: Vec<&str> = sanitized_name.split('-').collect();

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
                sanitized_name = format!("{}-{}", domain_prefix, new_parts.join("-"));
            }
            else {
                let max_length: usize = 40;
                let prefix_length: usize = 3;

                let start_index: usize = sanitized_name.len() - (max_length - prefix_length);
                let new_suffix: &str = &sanitized_name[start_index..];

                sanitized_name = format!("go-{}", new_suffix);
            }
        }

        SanitizedName {
            original: original_name.clone(),
            sanitized: sanitized_name,
        }
    }

    pub fn apply(&self, template: &String) -> String {
        template
            .replace("$NAME", &self.sanitized)
    }
}
