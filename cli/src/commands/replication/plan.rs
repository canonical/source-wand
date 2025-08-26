use std::{fs::create_dir_all, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use serde_json::Value;
use source_wand_common::{
    project_manipulator::{
        local_project_manipulator::LocalProjectManipulator,
        project_manipulator::ProjectManipulator
    },
    utils::{
        read_yaml_file::read_yaml_file
    }
};
use source_wand_dependency_analysis::{
    dependency_tree_node::DependencyTreeNode,
    dependency_tree_request::DependencyTreeRequest,
    find_dependency_tree
};
use source_wand_replication::model::{
    dependency::Dependency,
    package::Package,
    package_destination::PackageDestination,
    package_destination_git::PackageDestinationGit,
    package_origin::PackageOrigin,
    package_origin_go_cache::PackageOriginGoCache,
    replication_manifest::ReplicationManifest,
    replication_plan::ReplicationPlan
};
use uuid::Uuid;

#[derive(Debug, Parser)]
pub struct ReplicationPlanArgs;

pub fn replicate_plan_command(_args: &ReplicationPlanArgs) -> Result<()> {
    plan_replication()?;
    Ok(())
}

pub fn plan_replication() -> Result<ReplicationPlan> {
    let replication_manifest: ReplicationManifest = read_yaml_file("replication.yaml")?;

    match replication_manifest.origin {
        PackageOrigin::Git(origin) => {
            let uuid: Uuid = Uuid::new_v4();
            let top_level_directory: PathBuf = PathBuf::from(format!("./source-wand/{}", uuid));
            create_dir_all(&top_level_directory)?;
    
            let top_level: LocalProjectManipulator = LocalProjectManipulator::new(top_level_directory, true);
    
            top_level.run_shell(format!("git clone {} .", origin.git))?;
            top_level.run_shell(format!("git checkout {}", origin.reference))?;

            top_level.run_shell("go mod download all".to_string())?;

            let mut packages: Vec<Package> = Vec::new();

            let dependency_tree: DependencyTreeNode = find_dependency_tree(
                DependencyTreeRequest::from_git_project(
                    origin.git,
                    Some(origin.reference.clone())
                )
            )?;

            let build_dependencies: serde_json::Value = serde_json::from_str(
                top_level.run_shell(
                    "go list -json -m all | jq -s".to_string()
                )?.as_str()
            )?;

            if let Value::Array(build_dependencies) = build_dependencies {
                for build_dependency in build_dependencies {
                    if let Value::Object(build_dependency) = build_dependency {
                        let name: String = build_dependency.get("Path")
                            .unwrap()
                            .as_str()
                            .unwrap_or_default()
                            .replace("/", "-");

                        let version: String = build_dependency.get("Version")
                            .unwrap_or(
                                &Value::String(origin.reference.split('/')
                                    .last()
                                    .unwrap_or_default()
                                    .to_string()
                                )
                            )
                            .as_str()
                            .unwrap_or_default()
                            .to_string();

                        let cache_path: String = build_dependency.get("Dir")
                            .unwrap_or(&Value::String(String::new()))
                            .as_str()
                            .unwrap_or_default()
                            .to_string();

                        let environment: Environment = {
                            let mut major: String = String::new();
                            let mut minor: String = String::new();
                            let mut patch: String = String::new();
                            let mut suffix: String = String::new();

                            if version.starts_with('v') {
                                let parts: Vec<&str> = version.trim_start_matches('v').split('-').collect();
                                let semantic_version_parts: Vec<&str> = parts[0].split('.').collect();

                                if semantic_version_parts.len() > 0 {
                                    major = semantic_version_parts[0].to_string();
                                }
                                if semantic_version_parts.len() > 1 {
                                    minor = semantic_version_parts[1].to_string();
                                }
                                if semantic_version_parts.len() > 2 {
                                    patch = semantic_version_parts[2].to_string();
                                }

                                if parts.len() > 1 {
                                    suffix = format!("-{}", parts[1..].join("-"));
                                }
                            }

                            let retrocompatible: String =
                                if suffix.is_empty() {
                                    if major == "0".to_string() {
                                        format!("{}.{}.{}", major.clone(), minor, patch)
                                    }
                                    else {
                                        major.clone()
                                    }
                                }
                                else {
                                    format!("{}.{}.{}-{}", major, minor, patch, suffix)
                                };

                            Environment::new(name.clone(), version, major, minor, patch, suffix, retrocompatible)
                        };

                        let PackageDestination::Git(package_destination) = &replication_manifest.destination_template;
                        let package_destination_url: String = environment.apply(&package_destination.git);
                        let package_destination_reference: String = environment.apply(&package_destination.reference);

                        let dependencies: Vec<Dependency> = find_dependencies_for_package(
                            &dependency_tree,
                            &name,
                        );

                        let package: Package = Package::new(
                            PackageOriginGoCache::new(
                                environment.name,
                                environment.version,
                                cache_path
                            ),
                            PackageDestinationGit::new(
                                package_destination_url,
                                package_destination_reference,
                            ),
                            dependencies,
                        );

                        packages.push(package);
                    }
                }
            }
            top_level.cleanup();

            let replication_plan: ReplicationPlan = ReplicationPlan::new(replication_manifest.project, replication_manifest.hooks, packages);

            Ok(replication_plan)
        },
        PackageOrigin::GoCache(_origin) => { todo!() },
    }
}

struct Environment {
    name: String,
    version: String,
    version_major: String,
    version_minor: String,
    version_patch: String,
    version_suffix: String,
    version_retrocompatible: String,
}

impl Environment {
    pub fn new(
        name: String,
        version: String,
        version_major: String,
        version_minor: String,
        version_patch: String,
        version_suffix: String,
        version_retrocompatible: String,
    ) -> Self {
        Environment {
            name,
            version,
            version_major,
            version_minor,
            version_patch,
            version_suffix,
            version_retrocompatible
        }
    }

    pub fn apply(&self, template: &String) -> String {
        template
            .replace("$NAME", &self.name)
            .replace("$VERSION_MAJOR", &self.version_major)
            .replace("$VERSION_MINOR", &self.version_minor)
            .replace("$VERSION_PATCH", &self.version_patch)
            .replace("$VERSION_SUFFIX", &self.version_suffix)
            .replace("$VERSION_RETROCOMPATIBLE", &self.version_retrocompatible)
            .replace("$VERSION", &self.version)
    }
}

fn find_dependencies_for_package(root: &DependencyTreeNode, package_name: &str) -> Vec<Dependency> {
    if root.project.name.replace("/", "-") == package_name {
        return root.dependencies
            .iter()
            .map(|dep| Dependency {
                name: dep.project.name.replace("/", "-"),
                version: dep.project.version.clone(),
            })
            .collect();
    }

    for child in &root.dependencies {
        let found = find_dependencies_for_package(child, package_name);
        if !found.is_empty() {
            return found;
        }
    }

    Vec::new()
}
