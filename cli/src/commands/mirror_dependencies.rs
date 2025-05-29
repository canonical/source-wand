use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct MirrorDependenciesArgs {
    dependencies_file: PathBuf,
}

pub fn mirror_dependencies_command(args: &MirrorDependenciesArgs) -> Result<()> {
    println!("{:#?}", args.dependencies_file);
    Ok(())
}
