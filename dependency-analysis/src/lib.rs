use build_systems::identifier::identify_build_system;
use dependency_tree_generators::generate_dependency_tree;
use dependency_tree_node::DependencyTreeNode;
use dependency_tree_request::DependencyTreeRequest;
use source_wand_common::project_manipulator::{local_project_manipulator::LocalProjectManipulator, project_manipulator::AnyProjectManipulator};

pub mod dependency_tree_node;
pub mod dependency_tree_request;
pub mod build_systems;
pub mod dependency_tree_generators;

pub fn find_dependency_tree(request: DependencyTreeRequest) -> Result<DependencyTreeNode, String> {
    let project_manipulator: AnyProjectManipulator = match request {
        DependencyTreeRequest::LocalProject { path } => {
            LocalProjectManipulator::new(path).to_any()
        },
        _ => { todo!() },
        // DependencyTreeRequest::GitProject { url, branch } => {
        //     todo!()
        // },
        // DependencyTreeRequest::NameBased { name, version } => {
        //     todo!()
        // },
    };

    let build_system = identify_build_system(&project_manipulator)?;

    generate_dependency_tree(build_system, &project_manipulator)
}
