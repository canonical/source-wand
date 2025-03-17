use crate::project_manipulator::project_manipulator::{AnyProjectManipulator, ProjectManipulator};

use super::required_dependency::RequiredDependency;

pub struct PythonPipgripDependency;

impl RequiredDependency for PythonPipgripDependency {
    fn is_present(&self, project_manipulator: &AnyProjectManipulator) -> bool {
        match project_manipulator.run_shell("pipgrip".to_string()) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn install(&self, project_manipulator: &AnyProjectManipulator) -> Result<(), String> {
        println!("{}", project_manipulator.run_shell("pip install pipgrip".to_string())?);
        Ok(())
    }

    fn get_name(&self) -> String {
        "Pipgrip package for Python".to_string()
    }
}
