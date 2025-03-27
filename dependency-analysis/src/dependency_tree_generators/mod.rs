use java_maven_dependency_tree_generator::generate_java_maven_dependency_tree;
use python_pip_dependency_tree_generator::generate_python_pip_dependency_tree;
use source_wand_common::project_manipulator::project_manipulator::AnyProjectManipulator;
use rust_cargo_dependency_tree_generator::generate_rust_cargo_dependency_tree;

use crate::{
    build_systems::build_system_identity::BuildSystemIdentity,
    dependency_tree_node::DependencyTreeNode
};

pub mod rust_cargo_dependency_tree_generator;
pub mod python_pip_dependency_tree_generator;
pub mod java_maven_dependency_tree_generator;

pub fn generate_dependency_tree(
    build_system: BuildSystemIdentity,
    project_manipulator: &AnyProjectManipulator
) -> Result<DependencyTreeNode, String> {
    match build_system {
        BuildSystemIdentity::RustCargo => {
            generate_rust_cargo_dependency_tree(project_manipulator)
        },
        BuildSystemIdentity::PythonPip => {
            generate_python_pip_dependency_tree(project_manipulator)
        },
        BuildSystemIdentity::JavaMaven => {
            generate_java_maven_dependency_tree(project_manipulator)
        }
    }
}
