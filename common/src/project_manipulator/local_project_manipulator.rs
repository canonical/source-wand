use std::{path::PathBuf, process::Command};

use super::project_manipulator::ProjectManipulator;

#[derive(Debug, Clone)]
pub struct LocalProjectManipulator {
    pub project_root: PathBuf,
}

impl LocalProjectManipulator {
    pub fn new(project_root: PathBuf) -> Self {
        LocalProjectManipulator { project_root }
    }
}

impl ProjectManipulator for LocalProjectManipulator {
    fn run_shell(&self, command: String) -> Result<String, String> {
        let output = Command::new("bash")
            .arg("-c")
            .arg(&command)
            .current_dir(&self.project_root)
            .output()
            .map_err(|e| e.to_string())?;

        if output.status.success() {
            String::from_utf8(output.stdout)
                .map_err(|e| e.to_string())
        }
        else {
            Err(String::from_utf8(output.stderr).unwrap())
        }
    }

    fn try_run_shell(&self, command: String, retries: u32) -> Result<String, String> {
        self.to_any().try_run_shell(command, retries)
    }
    
    fn cleanup(&self) {}
}
