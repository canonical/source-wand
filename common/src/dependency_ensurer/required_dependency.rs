use anyhow::Result;

use crate::project_manipulator::project_manipulator::AnyProjectManipulator;

use super::{
    go::GoDependency,
    java_maven::JavaMavenDependency,
    python_pip::PythonPipDependency,
    python_pipgrip::PythonPipgripDependency,
    rust_cargo::RustCargoDependency
};

pub trait RequiredDependency {
    fn is_present(&self, project_manipulator: &AnyProjectManipulator) -> bool;
    fn install(&self, project_manipulator: &AnyProjectManipulator) -> Result<()>;

    fn get_name(&self) -> String;
}

pub enum AnyRequiredDependency {
    RustCargo(RustCargoDependency),
    PythonPip(PythonPipDependency),
    PythonPipgrip(PythonPipgripDependency),
    JavaMaven(JavaMavenDependency),
    Go(GoDependency),
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

impl GoDependency {
    pub fn to_any() -> AnyRequiredDependency {
        AnyRequiredDependency::Go(GoDependency)
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
            AnyRequiredDependency::Go(dependency) => {
                dependency.is_present(project_manipulator)
            },
        }
    }

    fn install(&self, project_manipulator: &AnyProjectManipulator) -> Result<()> {
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
            AnyRequiredDependency::Go(dependency) => {
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
            AnyRequiredDependency::Go(dependency) => {
                dependency.get_name()
            },
        }
    }
}
