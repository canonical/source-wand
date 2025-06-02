use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct OnboardArgs;

pub fn onboard_command(_args: &OnboardArgs) -> Result<()> {
    Ok(())
}
