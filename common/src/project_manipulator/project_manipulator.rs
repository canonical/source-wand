use super::{local_project_manipulator::LocalProjectManipulator, lxd_project_manipulator::LxdProjectManipulator};


pub trait ProjectManipulator {
    fn run_shell(&self, command: String) -> Result<String, String>;
}

pub enum AnyProjectManipulator {
    LocalManipulator(LocalProjectManipulator),
    LxdManipulator(LxdProjectManipulator),
}

impl LocalProjectManipulator {
    pub fn to_any(self) -> AnyProjectManipulator {
        AnyProjectManipulator::LocalManipulator(self)
    }
}

impl LxdProjectManipulator {
    pub fn to_any(self) -> AnyProjectManipulator {
        AnyProjectManipulator::LxdManipulator(self)
    }
}

impl ProjectManipulator for AnyProjectManipulator {
    fn run_shell(&self, command: String) -> Result<String, String> {
        match self {
            AnyProjectManipulator::LocalManipulator(project_manipulator) => {
                project_manipulator.run_shell(command)
            },
            AnyProjectManipulator::LxdManipulator(project_manipulator) => {
                project_manipulator.run_shell(command)
            },
        }
    }
}
