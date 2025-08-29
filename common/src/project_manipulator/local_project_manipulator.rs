use std::{path::PathBuf, process::Command, time::Duration};

use anyhow::{bail, Error, Result};
use tokio::time::sleep;

use super::project_manipulator::ProjectManipulator;
use async_trait::async_trait;

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
/////////////////////////////////////////////////////////////////////////////////////////////////
#[async_trait]
pub trait ProjectManipulatorAsync {
    /// Asynchronously runs a shell command in the project's directory.
    async fn run_shell(&self, command: String) -> Result<String>;

    /// Asynchronously tries to run a shell command with retries.
    async fn try_run_shell(&self, command: String, retries: u32) -> Result<String>;

    /// Returns the working directory path.
    fn get_working_directory(&self) -> PathBuf;

    /// Asynchronously cleans up the project directory.
    async fn cleanup(&self);
}

#[derive(Debug, Clone)]
pub struct LocalProjectManipulatorAsync {
    pub project_root: PathBuf,
    pub should_cleanup: bool,
}
impl LocalProjectManipulatorAsync {
    pub fn new(project_root: PathBuf, should_cleanup: bool) -> Self {
        LocalProjectManipulatorAsync { project_root, should_cleanup }
    }
}

#[async_trait]
impl ProjectManipulatorAsync for LocalProjectManipulatorAsync {
    async fn run_shell(&self, command: String) -> Result<String> {
        let output = Command::new("bash")
            .arg("-c")
            .arg(&command)
            .current_dir(&self.project_root)
            .output()?;
        if output.status.success() {
            Ok(String::from_utf8(output.stdout)?)
        } else {
            bail!(String::from_utf8(output.stderr)?)
        }
    }

    async fn try_run_shell(&self, command: String, retries: u32) -> Result<String> {
       let mut last_err = None;
        // We try a total of 'retries + 1' times.
        for attempt in 0..=retries {
            if attempt > 0 {
                // Wait before retrying. The delay increases with each attempt.
                // 1st retry: 100ms, 2nd: 200ms, 3rd: 400ms, etc.
                let delay_ms = 100 * 2_u64.pow(attempt - 1);
                let delay = Duration::from_millis(delay_ms);
                println!(
                    "Command failed. Retrying in {:?} (Attempt {}/{})",
                    delay, attempt, retries
                );
                sleep(delay).await;
            }

            match self.run_shell(command.clone()).await {
                Ok(output) => return Ok(output),
                Err(e) => {
                    last_err = Some(e);
                }
            }
        }
        // If all retries fail, return the last error we captured.
        Err(last_err.unwrap_or_else(|| {
            anyhow::anyhow!("Command failed after {} retries with no specific error.", retries)
        }))
    }

    fn get_working_directory(&self) -> PathBuf {
        self.project_root.clone()
    }

    async fn cleanup(&self) {
        if self.should_cleanup {
            // Use tokio's async fs operation for better performance than `rm -R`
            let _ = tokio::fs::remove_dir_all(&self.project_root).await;
        }
    }
}
