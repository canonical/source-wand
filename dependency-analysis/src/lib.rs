use std::{path::PathBuf, str::FromStr};

use build_systems::{
    build_system_identity::BuildSystemIdentity,
    identifier::identify_build_system
};
use dependency_tree_generators::generate_dependency_tree;
use dependency_tree_node::DependencyTreeNode;
use dependency_tree_request::DependencyTreeRequest;
use source_wand_common::{dependency_ensurer::required_dependency::AnyRequiredDependency, project_manipulator::{
        local_project_manipulator::LocalProjectManipulator,
        lxd_project_manipulator::LxdProjectManipulator,
        project_manipulator::{AnyProjectManipulator, ProjectManipulator}
}};
use uuid::Uuid;

pub mod dependency_tree_node;
pub mod dependency_tree_request;
pub mod build_systems;
pub mod dependency_tree_generators;

pub fn find_dependency_tree(request: DependencyTreeRequest) -> Result<DependencyTreeNode, String> {
    let project_manipulator: AnyProjectManipulator = match request {
        DependencyTreeRequest::LocalProject { path } => {
            LocalProjectManipulator::new(path).to_any()
        },
        DependencyTreeRequest::GitProject { url, branch } => {
            let machine_name: String = "source-wand-worker".to_string();
            let project_root: PathBuf = PathBuf::from_str(
                format!(
                    "/home/ubuntu/{}",
                    Uuid::new_v4().to_string()
                ).as_str()
            ).map_err(|e| e.to_string())?;
            let project_root_str: String = project_root.as_os_str().to_str().unwrap_or_default().to_string();

            let manipulator: LxdProjectManipulator = LxdProjectManipulator::new(machine_name, project_root)?;

            manipulator.try_run_shell(
                format!(
                    "git clone \"{}\" \"{}\"",
                    url,
                    project_root_str,
                ),
                20
            )?;
            if let Some(branch) = branch {
                manipulator.run_shell(format!("git checkout {}", branch))?;
            }

            manipulator.to_any()
        },
        _ => { todo!() },
        // DependencyTreeRequest::NameBased { name, version } => {
        //     todo!()
        // },
    };
    
    let build_system: BuildSystemIdentity = identify_build_system(&project_manipulator)?;

    let dependencies: Vec<AnyRequiredDependency> = build_system.get_required_dependencies();
    project_manipulator.ensure_dependencies(dependencies)?;

    let dependency_tree: Result<DependencyTreeNode, String> = generate_dependency_tree(build_system, &project_manipulator);
    project_manipulator.cleanup();

    dependency_tree
}
