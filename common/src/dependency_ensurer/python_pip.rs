use anyhow::Result;

use crate::project_manipulator::project_manipulator::{AnyProjectManipulator, ProjectManipulator};

use super::required_dependency::RequiredDependency;

pub struct PythonPipDependency;

impl RequiredDependency for PythonPipDependency {
    fn is_present(&self, project_manipulator: &AnyProjectManipulator) -> bool {
        match project_manipulator.run_shell("pip".to_string()) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn install(&self, project_manipulator: &AnyProjectManipulator) -> Result<()> {
        project_manipulator.run_shell("sudo apt-get update".to_string())?;
        project_manipulator.run_shell("sudo apt-get install -y python3.10".to_string())?;
        project_manipulator.run_shell("sudo apt-get install -y pip".to_string())?;
        Ok(())
    }

    fn get_name(&self) -> String {
        "Pip package manager for Python".to_string()
    }
}
