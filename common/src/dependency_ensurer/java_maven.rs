use crate::project_manipulator::project_manipulator::{AnyProjectManipulator, ProjectManipulator};

use super::required_dependency::RequiredDependency;

pub struct JavaMavenDependency;

impl RequiredDependency for JavaMavenDependency {
    fn is_present(&self, project_manipulator: &AnyProjectManipulator) -> bool {
        match project_manipulator.run_shell("mvn".to_string()) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn install(&self, project_manipulator: &AnyProjectManipulator) -> Result<(), String> {
        project_manipulator.run_shell("sudo apt-get update".to_string())?;
        project_manipulator.run_shell("sudo apt-get install -y maven".to_string())?;
        Ok(())
    }

    fn get_name(&self) -> String {
        "Maven package manager for Java".to_string()
    }
}
