use std::{fs, path::PathBuf};

use anyhow::Result;
use build_systems::{
    build_system_identity::BuildSystemIdentity,
    identifier::identify_build_system
};
use dependency_tree_generators::generate_dependency_tree;
use dependency_tree_node::DependencyTreeNode;
use dependency_tree_request::DependencyTreeRequest;
use source_wand_common::{
    dependency_ensurer::required_dependency::AnyRequiredDependency,
    project_manipulator::{
        local_project_manipulator::LocalProjectManipulator,
        project_manipulator::{AnyProjectManipulator, ProjectManipulator}
}};
use uuid::Uuid;

use crate::{build_requirements_generator::generate_build_requirements, unique_dependencies_list::UniqueDependenciesList};

pub mod dependency_tree_node;
pub mod dependency_tree_map;
pub mod unique_dependencies_list;

pub mod dependency_tree_request;

pub mod build_systems;

pub mod dependency_tree_generators;
pub mod build_requirements_generator;

pub fn find_dependency_tree(request: DependencyTreeRequest) -> Result<DependencyTreeNode> {
    let project_manipulator: AnyProjectManipulator = match request {
        DependencyTreeRequest::LocalProject { path } => {
            LocalProjectManipulator::new(path, false).to_any()
        },
        DependencyTreeRequest::GitProject { url, branch } => {
            let project_root: PathBuf = PathBuf::from(format!(
                "{}/source-wand-projects/{}",
                std::env::var("HOME")?,
                Uuid::new_v4().to_string()
            ));

            fs::create_dir_all(&project_root)?;

            let manipulator: LocalProjectManipulator = LocalProjectManipulator::new(project_root, true);

            manipulator.try_run_shell(
                format!(
                    "git clone \"{}\" .",
                    url,
                ),
                20
            )?;

            if let Some(branch) = branch {
                manipulator.run_shell(format!("git checkout {}", branch))?;
            }

            manipulator.to_any()
        },
        _ => { todo!() },
    };
    
    let build_system: BuildSystemIdentity = identify_build_system(&project_manipulator)?;

    // let dependencies: Vec<AnyRequiredDependency> = build_system.get_required_dependencies();
    // project_manipulator.ensure_dependencies(dependencies)?;

    let dependency_tree: Result<DependencyTreeNode> = generate_dependency_tree(build_system, &project_manipulator);
    project_manipulator.cleanup();

    dependency_tree
}


pub fn find_build_requirements(request: DependencyTreeRequest, dependency_tree: &DependencyTreeNode) -> Result<UniqueDependenciesList> {
    let project_manipulator: AnyProjectManipulator = match request {
        DependencyTreeRequest::LocalProject { path } => {
            LocalProjectManipulator::new(path, false).to_any()
        },
        DependencyTreeRequest::GitProject { url, branch } => {
            let project_root: PathBuf = PathBuf::from(format!(
                "{}/source-wand-projects/{}",
                std::env::var("HOME")?,
                Uuid::new_v4().to_string()
            ));

            fs::create_dir_all(&project_root)?;

            let manipulator: LocalProjectManipulator = LocalProjectManipulator::new(project_root, true);

            manipulator.try_run_shell(
                format!(
                    "git clone \"{}\" .",
                    url,
                ),
                20
            )?;

            if let Some(branch) = branch {
                manipulator.run_shell(format!("git checkout {}", branch))?;
            }

            manipulator.to_any()
        },
        _ => { todo!() },
    };
    
    let build_system: BuildSystemIdentity = identify_build_system(&project_manipulator)?;

    let dependencies: Vec<AnyRequiredDependency> = build_system.get_required_dependencies();
    project_manipulator.ensure_dependencies(dependencies)?;

    let build_requirements: Result<UniqueDependenciesList> = generate_build_requirements(build_system, &project_manipulator, dependency_tree);
    project_manipulator.cleanup();

    build_requirements
}
