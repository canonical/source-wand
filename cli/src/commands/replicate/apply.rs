use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct ReplicateApplyArgs;

pub fn replicate_apply_command(_args: &ReplicateApplyArgs) -> Result<()> {
    todo!()
}
