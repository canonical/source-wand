use source_wand_common::project_manipulator::project_manipulator::{AnyProjectManipulator, ProjectManipulator};

use super::build_system_identity::BuildSystemIdentity;

pub fn identify_build_system(project_manipulator: &AnyProjectManipulator) -> Result<BuildSystemIdentity, String> {
    if project_manipulator.run_shell("ls | grep \"^Cargo.toml$\"".to_string())?.trim() == "Cargo.toml" {
        return Ok(BuildSystemIdentity::RustCargo)
    }

    Err("Unable to identify the build system of the project.".to_string())
}
