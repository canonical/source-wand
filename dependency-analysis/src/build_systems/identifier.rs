use source_wand_common::project_manipulator::project_manipulator::{AnyProjectManipulator, ProjectManipulator};

use super::build_system_identity::BuildSystemIdentity;

pub fn identify_build_system(project_manipulator: &AnyProjectManipulator) -> Result<BuildSystemIdentity, String> {
    if project_manipulator.run_shell("ls | grep \"^Cargo.toml$\"".to_string()).unwrap_or_default().trim() == "Cargo.toml" {
        return Ok(BuildSystemIdentity::RustCargo);
    }

    if project_manipulator.run_shell("ls | grep \"^requirements.txt$\"".to_string()).unwrap_or_default().trim() == "requirements.txt" {
        return Ok(BuildSystemIdentity::PythonPip);
    }

    if project_manipulator.run_shell("ls | grep \"^pom.xml$\"".to_string()).unwrap_or_default().trim() == "pom.xml" {
        return Ok(BuildSystemIdentity::JavaMaven);
    }

    if project_manipulator.run_shell("ls | grep \"^go.mod$\"".to_string()).unwrap_or_default().trim() == "go.mod" {
        return Ok(BuildSystemIdentity::Go)
    }

    Err("Unable to identify the build system of the project.".to_string())
}
