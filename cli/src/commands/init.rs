use clap::Parser;

#[derive(Debug, Parser)]
pub struct InitArgs {
    #[arg(long)]
    from_git: String,
}

pub fn init_command(args: &InitArgs) -> Result<(), String> {
    Ok(())
}
