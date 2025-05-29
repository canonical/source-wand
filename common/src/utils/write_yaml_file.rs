use std::{fs::File, io::Write};

use anyhow::Result;
use serde::Serialize;

pub fn write_yaml_file<T: Serialize>(value: &T, file: &str) -> Result<()> {
    let yaml: String = serde_yaml::to_string(value)?;
    let mut file: File = File::create(file)?;

    file.write_all(yaml.as_bytes())?;

    Ok(())
}
