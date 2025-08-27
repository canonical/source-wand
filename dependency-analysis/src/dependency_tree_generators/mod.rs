use std::path::PathBuf;

use anyhow::Result;
use source_wand_common::{project, project_manipulator::project_manipulator::AnyProjectManipulator};

use crate::{
    build_systems::build_system_identity::BuildSystemIdentity,
    dependency_tree_generators::{cdxgen_dependency_tree_generator::generate_cdxgen_dependency_tree, go_dependency_tree_generator::generate_go_dependency_tree},
    dependency_tree_node::DependencyTreeNode
};

pub mod rust_cargo_dependency_tree_generator;
pub mod python_pip_dependency_tree_generator;
pub mod java_maven_dependency_tree_generator;
pub mod go_dependency_tree_generator;
pub mod go_dependency_tree_generator_andrew;
pub mod go_depenendency_tree_struct;

pub mod cdxgen_dependency_tree_generator;
pub mod cdxgen_rust_dependency_tree_generator;
pub mod cdxgen_python_dependency_tree_generator;
pub mod cdxgen_java_dependency_tree_generator;
pub mod cdxgen_go_dependency_tree_generator;

pub fn generate_dependency_tree(
    build_system: BuildSystemIdentity,
    project_manipulator: &AnyProjectManipulator,
    //project_root: &PathBuf
) -> Result<DependencyTreeNode> {
    match build_system {
        BuildSystemIdentity::RustCargo => {
            generate_cdxgen_dependency_tree(project_manipulator, Some("rust"))
        },
        BuildSystemIdentity::PythonPip => {
            generate_cdxgen_dependency_tree(project_manipulator, Some("python"))
        },
        BuildSystemIdentity::JavaMaven => {
            generate_cdxgen_dependency_tree(project_manipulator, Some("java"))
        },
        BuildSystemIdentity::Go => {
            //generate_cdxgen_dependency_tree(project_manipulator, Some("go"))
            generate_go_dependency_tree(project_manipulator)
        },
        BuildSystemIdentity::Unknown => {
            generate_cdxgen_dependency_tree(project_manipulator, None)
        }
    }
}
