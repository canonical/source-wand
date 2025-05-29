use anyhow::Result;
use source_wand_common::utils::read_yaml_file::read_yaml_file;
use source_wand_dependency_analysis::dependency_tree_node::DependencyTreeNode;

pub fn plan_onboarding() -> Result<()> {
    let dependency_tree: DependencyTreeNode = read_yaml_file("dependencies.yaml")?;
    Ok(())
}
