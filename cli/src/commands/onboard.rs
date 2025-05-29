use clap::Parser;

#[derive(Debug, Parser)]
pub struct OnboardArgs {
    #[arg(long)]
    from_git: String,
}

pub fn onboard_command(args: &OnboardArgs) -> Result<(), String> {
    Ok(())
}
