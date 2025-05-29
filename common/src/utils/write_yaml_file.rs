use std::{fs::{create_dir_all, File}, io::Write, path::Path};

use anyhow::Result;
use serde::Serialize;

pub fn write_yaml_file<T: Serialize>(value: &T, file: &str) -> Result<()> {
    if let Some(parent_directories) = Path::new(file).parent() {
        create_dir_all(parent_directories)?;
    }

    let yaml: String = serde_yaml::to_string(value)?;
    let mut file: File = File::create(file)?;

    file.write_all(yaml.as_bytes())?;

    Ok(())
}
