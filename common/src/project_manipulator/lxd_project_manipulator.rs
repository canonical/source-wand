use std::{path::PathBuf, process::Command, str::FromStr, thread, time::Duration};

use super::{local_project_manipulator::LocalProjectManipulator, project_manipulator::ProjectManipulator};

#[derive(Debug, Clone)]
pub struct LxdProjectManipulator {
    pub machine_name: String,
    pub project_root: PathBuf,
}

impl LxdProjectManipulator {
    pub fn new(machine_name: String, project_root: PathBuf) -> Result<Self, String> {
        if !Self::machine_exists(&machine_name) {
            println!("Machine \"{}\" does not exist, creating it.", machine_name);
            Self::create_machine(&machine_name, "ubuntu:22.04")?;
        }

        Ok(
            LxdProjectManipulator {
                machine_name,
                project_root,
            }
        )
    }

    fn machine_exists(machine_name: &String) -> bool {
        let local: LocalProjectManipulator = LocalProjectManipulator::new(PathBuf::from_str("/").unwrap());
        let lxc_machine: String = local.run_shell(format!("lxc list --format json | jq '.[] | .name' | grep \"^\\\"{}\\\"$\"", machine_name))
                                       .unwrap_or_default()
                                       .trim()
                                       .to_string();

        lxc_machine == format!("\"{}\"", machine_name)
    }

    fn create_machine(machine_name: &String, base: &str) -> Result<(), String> {
        let local: LocalProjectManipulator = LocalProjectManipulator::new(PathBuf::from_str("/").unwrap());
        let response: Result<(), String> = local.run_shell(format!("lxc launch {} {}", base, machine_name))
            .map(|_| ())
            .map_err(|e| e.to_string());

        thread::sleep(Duration::from_secs(5));

        response
    }
}

impl ProjectManipulator for LxdProjectManipulator {
    fn run_shell(&self, command: String) -> Result<String, String> {
        let output = Command::new("bash")
            .arg("-c")
            .arg(format!(
                "lxc exec {} -- bash << EOF\ncd {}\n{}\nEOF",
                self.machine_name,
                self.project_root.as_os_str().to_str().unwrap_or_default(),
                command
            ))
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
    
    fn cleanup(&self) {
        let _ = self.run_shell(
            format!(
                "rm -R {}",
                self.project_root.as_os_str().to_str().unwrap_or_default()
            )
        );
    }
}
