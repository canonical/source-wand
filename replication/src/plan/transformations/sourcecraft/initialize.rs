use std::collections::HashMap;

use anyhow::Result;
use serde::Serialize;
use source_wand_common::{project_manipulator::project_manipulator::ProjectManipulator, utils::write_yaml_file::write_yaml_file};

use crate::{model::dependency::Dependency, plan::{context::Context, transformation::Transformation}};

#[derive(Debug, Clone)]
pub struct SourcecraftInitialize {
    pub name: String,
    pub version: String,
    pub base: String,
    pub platforms: Vec<String>,
    pub dependencies: Vec<Dependency>,
}

impl SourcecraftInitialize {
    pub fn new(
        name: String,
        version: String,
        base: String,
        platforms: Vec<String>,
        dependencies: Vec<Dependency>,
    ) -> Self {
        SourcecraftInitialize {
            name,
            version,
            base,
            platforms,
            dependencies,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
struct SourcecraftMetadata {
    pub name: String,
    pub version: String,
    pub base: String,
    pub summary: String,
    pub description: String,
    pub platforms: HashMap<String, ()>,
    pub parts: HashMap<String, Part>
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
enum Part {
    Go(GoPart),
    GoUse(GoUsePart),
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
struct GoPart {
    pub plugin: String,
    pub source: String,
    pub build_snaps: Option<Vec<String>>,
    pub build_environment: Option<Vec<HashMap<String, String>>>,
    pub after: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
struct GoUsePart {
    pub plugin: String,
    pub source: String,
    pub source_channel: Option<String>,
}

impl Part {
    pub fn with_go_plugin(
        source: String,
        build_snaps: Vec<String>,
        build_environment: Vec<HashMap<String, String>>,
        after: Vec<String>,
    ) -> Self {
        Part::Go(
            GoPart {
                plugin: "go".to_string(),
                source,
                build_snaps: Some(build_snaps),
                build_environment: Some(build_environment),
                after: Some(after),
            }
        )
    }

    pub fn with_go_use_plugin(
        name: String,
        track: String,
    ) -> Self {
        Part::GoUse(
            GoUsePart {
                plugin: "go-use".to_string(),
                source: format!("sourcecraft:{}", name),
                source_channel: Some(format!("{}/edge", track)),
            }
        )
    }
}

impl SourcecraftMetadata {
    pub fn from_args(args: &SourcecraftInitialize) -> Self {
        let mut parts: HashMap<String, Part> = HashMap::new();

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

impl Transformation for SourcecraftInitialize {
    fn apply(&self, ctx: Context) -> Result<Context> {
        let sourcecraft_metadata: SourcecraftMetadata = SourcecraftMetadata::from_args(&self);
        write_yaml_file(
            &sourcecraft_metadata,
            format!("{}/sourcecraft.yaml", ctx.sh.project_root.to_str().unwrap()).as_str(),
        )?;
        Ok(ctx)
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
