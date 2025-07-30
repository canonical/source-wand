use std::path::PathBuf;

use anyhow::{Error, Result};

use crate::{
    dependency_ensurer::required_dependency::{
        AnyRequiredDependency,
        RequiredDependency
    }
};

use super::{
    local_project_manipulator::LocalProjectManipulator,
    lxd_project_manipulator::LxdProjectManipulator
};

pub trait ProjectManipulator {
    fn run_shell(&self, command: String) -> Result<String>;
    fn try_run_shell(&self, command: String, retries: u32) -> Result<String>;
    fn get_working_directory(&self) -> PathBuf;
    fn cleanup(&self);
}

pub enum AnyProjectManipulator {
    LocalManipulator(LocalProjectManipulator),
    LxdManipulator(LxdProjectManipulator),
}

impl LocalProjectManipulator {
    pub fn to_any(&self) -> AnyProjectManipulator {
        AnyProjectManipulator::LocalManipulator(self.clone())
    }
}

impl LxdProjectManipulator {
    pub fn to_any(&self) -> AnyProjectManipulator {
        AnyProjectManipulator::LxdManipulator(self.clone())
    }
}

impl ProjectManipulator for AnyProjectManipulator {
    fn run_shell(&self, command: String) -> Result<String> {
        match self {
            AnyProjectManipulator::LocalManipulator(project_manipulator) => {
                project_manipulator.run_shell(command)
            },
            AnyProjectManipulator::LxdManipulator(project_manipulator) => {
                project_manipulator.run_shell(command)
            },
        }
    }

    fn try_run_shell(&self, command: String, retries: u32) -> Result<String> {
        let mut error: Error = Error::msg("Failed to run command.");

        for _ in 0..retries {
            match self.run_shell(command.clone()) {
                Ok(response) => {
                    return Ok(response);
                },
                Err(e) => {
                    error = e;
                },
            }
        }

        Err(error)
    }

    fn get_working_directory(&self) -> PathBuf {
        match self {
            AnyProjectManipulator::LocalManipulator(project_manipulator) => {
                project_manipulator.get_working_directory()
            },
            AnyProjectManipulator::LxdManipulator(project_manipulator) => {
                project_manipulator.get_working_directory()
            },
        }
    }

    fn cleanup(&self) {
        match self {
            AnyProjectManipulator::LocalManipulator(project_manipulator) => {
                project_manipulator.cleanup();
            },
            AnyProjectManipulator::LxdManipulator(project_manipulator) => {
                project_manipulator.cleanup();
            },
        }
    }
}

impl AnyProjectManipulator {
    pub fn ensure_dependencies(&self, dependencies: Vec<AnyRequiredDependency>) -> Result<()> {
        for dependency in dependencies {
            if !dependency.is_present(&self) {
                println!("Dependency \"{}\" is required to run this command. Installing.", dependency.get_name());
                dependency.install(&self)
                    .map_err(|e| anyhow::format_err!("Failed to install dependency: {}", e))?;
            }
        }

        Ok(())
    }
}
