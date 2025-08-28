use std::collections::HashMap;

use anyhow::Result;

use source_wand_common::{
    project_manipulator::project_manipulator::ProjectManipulator,
    utils::write_yaml_file::write_yaml_file
};

use crate::{
    model::{
        dependency::Dependency,
        sourcecraft::{
            part::Part,
            sourcecraft_metadata::SourcecraftMetadata
        }
    },
    plan::{
        context::Context,
        transformation::Transformation
    }
};

#[derive(Debug, Clone)]
pub struct SourcecraftInitialize {
    pub name: String,
    pub version: String,
    pub base: String,
    pub platforms: Vec<String>,
    pub dependencies: Vec<Dependency>,

    pub is_library: bool,
}

impl SourcecraftInitialize {
    pub fn new(
        name: String,
        version: String,
        base: String,
        platforms: Vec<String>,
        dependencies: Vec<Dependency>,
        is_library: bool,
    ) -> Self {
        SourcecraftInitialize {
            name,
            version,
            base,
            platforms,
            dependencies,
            is_library,
        }
    }
}

impl Transformation for SourcecraftInitialize {
    fn apply(&self, ctx: Context) -> Result<Option<String>> {
        let sourcecraft_metadata: SourcecraftMetadata = SourcecraftMetadata::from_args(&self);
        write_yaml_file(
            &sourcecraft_metadata,
            format!("{}/sourcecraft.yaml", ctx.sh.project_root.to_str().unwrap()).as_str(),
        )?;
        Ok(None)
    }

    fn should_skip(&self, ctx: &Context) -> Option<String> {
        if ctx.sh.run_shell(
            "ls | grep \"^sourcecraft.yaml$\"".to_string()
        ).unwrap_or_default().trim() == "sourcecraft.yaml" {
            Some("sourcecraft.yaml already exists".to_string())
        }
        else {
            None
        }
    }

    fn get_name(&self) -> String {
        "initialize sourcecraft project".to_string()
    }
}


impl SourcecraftMetadata {
    pub fn from_args(args: &SourcecraftInitialize) -> Self {
        let mut parts: HashMap<String, Part> = HashMap::new();

        if args.is_library {
            parts.insert(
                args.name.clone(),
                Part::with_nil_plugin(),
            );
        }
        else {
            for dependency in &args.dependencies {
                parts.insert(
                    dependency.name.clone(),
                    Part::with_go_use_plugin(
                        dependency.name.clone(),
                        dependency.version.clone()
                    )
                );
            }
    
            parts.insert(
                args.name.clone(),
                Part::with_go_plugin(
                    ".".to_string(),
                    vec!["go".to_string()],
                    [
                        ("GOFLAGS".to_string(), "-json".to_string()),
                        ("GOPROXY".to_string(), "False".to_string()),
                    ].iter()
                     .map(|(key, value)| {
                        let mut map: HashMap<String, String> = HashMap::new();
                        map.insert(key.clone(), value.clone());
                        map
                     }).collect(),
                    args.dependencies
                        .iter()
                        .map(|dependency| dependency.name.clone())
                        .collect()
                )
            );
        }

        SourcecraftMetadata {
            name: args.name.clone(),
            version: args.version.clone(),
            base: args.base.clone(),
            summary: format!("{} version {} (Golang program)", args.name.clone(), args.version.clone()),
            description: format!("{} version {} (Golang program), onboarded by source-wand", args.name.clone(), args.version.clone()),
            platforms: args.platforms.clone()
                .into_iter()
                .map(|platform| (platform.clone(), ()))
                .collect(),
            parts
        }
    }
}
