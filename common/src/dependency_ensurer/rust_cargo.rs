use anyhow::Result;

use crate::project_manipulator::project_manipulator::{AnyProjectManipulator, ProjectManipulator};

use super::required_dependency::RequiredDependency;

pub struct RustCargoDependency;

impl RequiredDependency for RustCargoDependency {
    fn is_present(&self, project_manipulator: &AnyProjectManipulator) -> bool {
        match project_manipulator.run_shell("cargo".to_string()) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn install(&self, project_manipulator: &AnyProjectManipulator) -> Result<()> {
        project_manipulator.run_shell("sudo snap install rustup --classic".to_string())?;
        project_manipulator.run_shell("rustup default stable".to_string())?;
        Ok(())
    }

    fn get_name(&self) -> String {
        "Cargo toolchain for Rust".to_string()
    }
}
