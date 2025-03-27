use crate::project_manipulator::project_manipulator::AnyProjectManipulator;

use super::{
    java_maven::JavaMavenDependency, python_pip::PythonPipDependency, python_pipgrip::PythonPipgripDependency, rust_cargo::RustCargoDependency
};

pub trait RequiredDependency {
    fn is_present(&self, project_manipulator: &AnyProjectManipulator) -> bool;
    fn install(&self, project_manipulator: &AnyProjectManipulator) -> Result<(), String>;

    fn get_name(&self) -> String;
}

pub enum AnyRequiredDependency {
    RustCargo(RustCargoDependency),
    PythonPip(PythonPipDependency),
    PythonPipgrip(PythonPipgripDependency),
    JavaMaven(JavaMavenDependency),
}

impl RustCargoDependency {
    pub fn to_any() -> AnyRequiredDependency {
        AnyRequiredDependency::RustCargo(RustCargoDependency)
    }
}

impl PythonPipDependency {
    pub fn to_any() -> AnyRequiredDependency {
        AnyRequiredDependency::PythonPip(PythonPipDependency)
    }
}

impl PythonPipgripDependency {
    pub fn to_any() -> AnyRequiredDependency {
        AnyRequiredDependency::PythonPipgrip(PythonPipgripDependency)
    }
}

impl JavaMavenDependency {
    pub fn to_any() -> AnyRequiredDependency {
        AnyRequiredDependency::JavaMaven(JavaMavenDependency)
    }
}

impl RequiredDependency for AnyRequiredDependency {
    fn is_present(&self, project_manipulator: &AnyProjectManipulator) -> bool {
        match self {
            AnyRequiredDependency::RustCargo(dependency) => {
                dependency.is_present(project_manipulator)
            },
            AnyRequiredDependency::PythonPip(dependency) => {
                dependency.is_present(project_manipulator)
            },
            AnyRequiredDependency::PythonPipgrip(dependency) => {
                dependency.is_present(project_manipulator)
            },
            AnyRequiredDependency::JavaMaven(dependency) => {
                dependency.is_present(project_manipulator)
            },
        }
    }

    fn install(&self, project_manipulator: &AnyProjectManipulator) -> Result<(), String> {
        match self {
            AnyRequiredDependency::RustCargo(dependency) => {
                dependency.install(project_manipulator)
            },
            AnyRequiredDependency::PythonPip(dependency) => {
                dependency.install(project_manipulator)
            },
            AnyRequiredDependency::PythonPipgrip(dependency) => {
                dependency.install(project_manipulator)
            },
            AnyRequiredDependency::JavaMaven(dependency) => {
                dependency.install(project_manipulator)
            },
        }
    }
    
    fn get_name(&self) -> String {
        match self {
            AnyRequiredDependency::RustCargo(dependency) => {
                dependency.get_name()
            },
            AnyRequiredDependency::PythonPip(dependency) => {
                dependency.get_name()
            },
            AnyRequiredDependency::PythonPipgrip(dependency) => {
                dependency.get_name()
            },
            AnyRequiredDependency::JavaMaven(dependency) => {
                dependency.get_name()
            },
        }
    }
}
