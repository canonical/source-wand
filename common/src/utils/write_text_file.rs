use std::{fs::{create_dir_all, File}, io::Write, path::Path};

use anyhow::Result;

pub fn write_text_file(contents: &String, file: &str) -> Result<()> {
    if let Some(parent_directories) = Path::new(file).parent() {
        create_dir_all(parent_directories)?;
    }

    let mut file: File = File::create(file)?;
    file.write_all(contents.as_bytes())?;

    Ok(())
}
