use std::{fs::File, io::Read};

use anyhow::Result;
use serde::de::DeserializeOwned;

pub fn read_yaml_file<T: DeserializeOwned>(file: &str) -> Result<T> {
    let mut file: File = File::open(file)?;
    let mut yaml: String = String::new();

    file.read_to_string(&mut yaml)?;    
    let value: T = serde_yaml::from_str(&yaml)?;

    Ok(value)
}
