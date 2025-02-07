use crate::project_manipulator::project_manipulator::AnyProjectManipulator;

use super::rust_cargo::RustCargoDependency;

pub trait RequiredDependency {
    fn is_present(&self, project_manipulator: &AnyProjectManipulator) -> bool;
    fn install(&self, project_manipulator: &AnyProjectManipulator) -> Result<(), String>;

    fn get_name(&self) -> String;
}

pub enum AnyRequiredDependency {
    RustCargo(RustCargoDependency),
}

impl RustCargoDependency {
    pub fn to_any() -> AnyRequiredDependency {
        AnyRequiredDependency::RustCargo(RustCargoDependency)
    }
}

impl RequiredDependency for AnyRequiredDependency {
    fn is_present(&self, project_manipulator: &AnyProjectManipulator) -> bool {
        match self {
            AnyRequiredDependency::RustCargo(dependency) => {
                dependency.is_present(project_manipulator)
            },
        }
    }

    fn install(&self, project_manipulator: &AnyProjectManipulator) -> Result<(), String> {
        match self {
            AnyRequiredDependency::RustCargo(dependency) => {
                dependency.install(project_manipulator)
            }
        }
    }
    
    fn get_name(&self) -> String {
        match self {
            AnyRequiredDependency::RustCargo(dependency) => {
                dependency.get_name()
            }
        }
    }
}
