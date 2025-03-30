use source_wand_common::dependency_ensurer::{
    go::GoDependency,
    python_pip::PythonPipDependency,
    python_pipgrip::PythonPipgripDependency,
    required_dependency::AnyRequiredDependency,
    rust_cargo::RustCargoDependency
};

pub enum BuildSystemIdentity {
    RustCargo,
    PythonPip,
    Go,
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
            BuildSystemIdentity::Go => {
                vec![
                    GoDependency::to_any(),
                ]
            },
        }
    }
}
