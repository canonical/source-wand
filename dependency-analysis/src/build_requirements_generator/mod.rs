use anyhow::Result;

use source_wand_common::project_manipulator::project_manipulator::AnyProjectManipulator;

use crate::{
    build_requirements_generator::go_build_requirements_generator::generate_go_build_requirements,
    build_systems::build_system_identity::BuildSystemIdentity,
    dependency_tree_node::DependencyTreeNode,
    unique_dependencies_list::UniqueDependenciesList
};

pub mod go_build_requirements_generator;

pub fn generate_build_requirements(
    build_system: BuildSystemIdentity,
    project_manipulator: &AnyProjectManipulator,
    dependency_tree: &DependencyTreeNode,
) -> Result<UniqueDependenciesList> {
    match build_system {
        // BuildSystemIdentity::RustCargo => {
            //     generate_rust_cargo_dependency_tree(project_manipulator)
            // },
            // BuildSystemIdentity::PythonPip => {
                //     generate_python_pip_dependency_tree(project_manipulator)
                // },
                // BuildSystemIdentity::JavaMaven => {
                    //     generate_java_maven_dependency_tree(project_manipulator)
        // },
        BuildSystemIdentity::Go => {
            generate_go_build_requirements(project_manipulator, dependency_tree)
        },
        _ => { todo!() },
    }
}
