use anyhow::Result;

use crate::project_manipulator::project_manipulator::{AnyProjectManipulator, ProjectManipulator};

use super::required_dependency::RequiredDependency;

pub struct GoDependency;

impl RequiredDependency for GoDependency {
    fn is_present(&self, project_manipulator: &AnyProjectManipulator) -> bool {
        match project_manipulator.run_shell("go help".to_string()) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn install(&self, project_manipulator: &AnyProjectManipulator) -> Result<()> {
        project_manipulator.run_shell("sudo snap install go --classic".to_string())?;
        Ok(())
    }

    fn get_name(&self) -> String {
        "Go CLI tool".to_string()
    }
}
