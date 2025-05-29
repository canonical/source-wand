use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct CompareArgs {
    first_file: PathBuf,
    second_file: PathBuf,
}

pub fn compare_command(args: &CompareArgs) -> Result<()> {
    println!("{:#?} {:#?}", args.first_file, args.second_file);
    Ok(())
}
