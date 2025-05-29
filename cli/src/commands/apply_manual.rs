use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct ApplyManualArgs {
    #[arg(long)]
    from_git: String,
}

pub fn apply_manual_command(_args: &ApplyManualArgs) -> Result<()> {
    Ok(())
}
