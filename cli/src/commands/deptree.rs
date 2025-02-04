use std::path::PathBuf;

use clap::Parser;
use source_wand_dependency_analysis::find_dependency_tree;


#[derive(Debug, Parser)]
pub struct DeptreeArgs {
    path: PathBuf,
}

pub fn deptree_command(args: &DeptreeArgs) -> Result<(), String> {
    let dependency_tree = find_dependency_tree(
        source_wand_dependency_analysis::dependency_tree_request::DependencyTreeRequest::LocalProject {
            path: args.path.clone()
        }
    )?;

    println!("{}", dependency_tree.to_string()?);

    Ok(())
}
