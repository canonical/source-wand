use source_wand_common::dependency_ensurer::{required_dependency::AnyRequiredDependency, rust_cargo::RustCargoDependency};

pub enum BuildSystemIdentity {
    RustCargo,
}

impl BuildSystemIdentity {
    pub fn get_required_dependencies(&self) -> Vec<AnyRequiredDependency> {
        match self {
            BuildSystemIdentity::RustCargo => {
                vec![
                    RustCargoDependency::to_any(),
                ]
            }
        }
    }
}
