use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct MirrorDependenciesArgs {
    dependencies_file: PathBuf,
}

pub fn mirror_dependencies_command(args: &MirrorDependenciesArgs) -> Result<(), String> {
    println!("{:#?}", args.dependencies_file);
    // let dependencies = 
    Ok(())
}
