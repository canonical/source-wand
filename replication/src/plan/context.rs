use source_wand_common::project_manipulator::local_project_manipulator::LocalProjectManipulator;

#[derive(Debug, Clone)]
pub struct Context {
    pub sh: LocalProjectManipulator,
}

impl Context {
    pub fn new(sh: LocalProjectManipulator) -> Self {
        Context { sh }
    }
}
