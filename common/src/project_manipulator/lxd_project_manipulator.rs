use std::{path::PathBuf, process::Command};

use super::project_manipulator::ProjectManipulator;

pub struct LxdProjectManipulator {
    pub machine_name: String,
    pub project_root: PathBuf,
}

impl LxdProjectManipulator {
    pub fn new(machine_name: String, project_root: PathBuf) -> Self {
        LxdProjectManipulator { machine_name, project_root }
    }
}

impl ProjectManipulator for LxdProjectManipulator {
    fn run_shell(&self, command: String) -> Result<String, String> {
        let output = Command::new("bash")
            .arg("-c")
            .arg(format!("lxc exec {} -- ", self.machine_name))
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
}
