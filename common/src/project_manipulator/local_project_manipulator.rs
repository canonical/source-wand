use std::{path::PathBuf, process::Command};

use anyhow::{bail, Error, Result};

use super::project_manipulator::ProjectManipulator;

#[derive(Debug, Clone)]
pub struct LocalProjectManipulator {
    pub project_root: PathBuf,
    pub should_cleanup: bool,
}

impl LocalProjectManipulator {
    pub fn new(project_root: PathBuf, should_cleanup: bool) -> Self {
        LocalProjectManipulator { project_root, should_cleanup }
    }
}

impl ProjectManipulator for LocalProjectManipulator {
    fn run_shell(&self, command: String) -> Result<String> {
        let output = Command::new("bash")
            .arg("-c")
            .arg(&command)
            .current_dir(&self.project_root)
            .output()?;

        if output.status.success() {
            String::from_utf8(output.stdout)
                .map_err(|e| Error::msg(e))
        }
        else {
            bail!(String::from_utf8(output.stderr).unwrap())
        }
    }

    fn try_run_shell(&self, command: String, retries: u32) -> Result<String> {
        self.to_any().try_run_shell(command, retries)
    }

    fn get_working_directory(&self) -> PathBuf {
        self.project_root.clone()
    }

    fn cleanup(&self) {
        if self.should_cleanup {
            let _ = self.run_shell(
                format!(
                    "rm -R {}",
                    self.project_root.as_os_str().to_str().unwrap_or_default()
                )
            );
        }
    }
}
