use anyhow::{Error, Result};
use clap::Parser;
use source_wand_common::utils::write_yaml_file::write_yaml_file;
use source_wand_dependency_analysis::{dependency_tree_node::DependencyTreeNode, dependency_tree_request::DependencyTreeRequest, find_dependency_tree};
use source_wand_onboarding::plan::plan_onboarding;

#[derive(Debug, Parser)]
pub struct InitArgs {
    #[arg(long)]
    from_git: String,

    #[arg(long)]
    checkout: Option<String>,
}

pub fn init_command(args: &InitArgs) -> Result<()> {
    println!("Planning project onboarding");
    println!(" > Generating dependency tree");

    let dependency_tree: DependencyTreeNode = find_dependency_tree(
        DependencyTreeRequest::GitProject {
            url: args.from_git.clone(),
            branch: args.checkout.clone(),
        }
    ).map_err(|e| Error::msg(e))?;

    println!(" > Saving dependency tree");
    write_yaml_file(&dependency_tree, "dependencies.yaml")?;

    plan_onboarding()?;

    Ok(())
}
