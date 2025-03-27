use source_wand_common::dependency_ensurer::{
    java_maven::JavaMavenDependency, python_pip::PythonPipDependency, python_pipgrip::PythonPipgripDependency, required_dependency::AnyRequiredDependency, rust_cargo::RustCargoDependency
};

pub enum BuildSystemIdentity {
    RustCargo,
    PythonPip,
    JavaMaven,
}

impl BuildSystemIdentity {
    pub fn get_required_dependencies(&self) -> Vec<AnyRequiredDependency> {
        match self {
            BuildSystemIdentity::RustCargo => {
                vec![
                    RustCargoDependency::to_any(),
                ]
            },
            BuildSystemIdentity::PythonPip => {
                vec![
                    PythonPipDependency::to_any(),
                    PythonPipgripDependency::to_any(),
                ]
            },
            BuildSystemIdentity::JavaMaven => {
                vec![
                    JavaMavenDependency::to_any(),
                ]
            }
        }
    }
}
