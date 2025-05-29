use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct OnboardArgs {
    #[arg(long)]
    from_git: String,
}

pub fn onboard_command(_args: &OnboardArgs) -> Result<()> {
    Ok(())
}
