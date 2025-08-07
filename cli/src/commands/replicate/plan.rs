use std::{fs::create_dir_all, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use serde_json::Value;
use source_wand_common::{project_manipulator::{local_project_manipulator::LocalProjectManipulator, project_manipulator::ProjectManipulator}, utils::{read_yaml_file::read_yaml_file, write_yaml_file::write_yaml_file}};
use source_wand_replication::model::{package::Package, package_destination::PackageDestination, package_destination_git::PackageDestinationGit, package_origin::PackageOrigin, package_origin_go_cache::PackageOriginGoCache, replication_manifest::ReplicationManifest, replication_plan::ReplicationPlan};

#[derive(Debug, Parser)]
pub struct ReplicatePlanArgs;

pub fn replicate_plan_command(_args: &ReplicatePlanArgs) -> Result<()> {
    let replication_manifest: ReplicationManifest = read_yaml_file("replication.yaml")?;

    match replication_manifest.origin {
        PackageOrigin::Git(origin) => {
            let top_level_directory: PathBuf = PathBuf::from("./source-wand/top-level/repository");
            create_dir_all(&top_level_directory)?;
    
            let top_level: LocalProjectManipulator = LocalProjectManipulator::new(top_level_directory, true);
    
            top_level.run_shell(format!("git clone {} .", origin.git))?;
            top_level.run_shell(format!("git checkout {}", origin.reference))?;

            let mut packages: Vec<Package> = Vec::new();

            let build_dependencies: serde_json::Value = serde_json::from_str(top_level.run_shell("go list -json -m all | jq -s".to_string())?.as_str())?;
            if let Value::Array(build_dependencies) = build_dependencies {
                for build_dependency in build_dependencies {
                    if let Value::Object(build_dependency) = build_dependency {
                        println!("name: {:#?}", build_dependency.get("Path"));
                        println!("version: {:#?}", build_dependency.get("Version"));
                        println!("cache_path: {:#?}", build_dependency.get("GoMod"));
                        let name: String = build_dependency.get("Path").unwrap().as_str().unwrap_or_default().replace("/", "-");
                        let version: String = build_dependency.get("Version").unwrap_or(&Value::String(String::new())).as_str().unwrap_or_default().to_string();
                        let cache_path: String = build_dependency.get("GoMod").unwrap_or(&Value::String(String::new())).as_str().unwrap_or_default().to_string();
                        let PackageDestination::Git(package_destination) = &replication_manifest.destination_template;
                        let package_destination_url: String = package_destination.git
                            .replace("$NAME", &name)
                            .replace("$VERSION", &version);
                        let package_destination_reference: String = package_destination.reference
                            .replace("$NAME", &name)
                            .replace("$VERSION", &version);

                        let package: Package = Package::new(
                            0,
                            PackageOriginGoCache::new(name, version, cache_path),
                            PackageDestinationGit::new(
                                package_destination_url,
                                package_destination_reference,
                            ),
                            Vec::new(),
                        );

                        packages.push(package);
                    }
                }
            }

            let replication_plan: ReplicationPlan = ReplicationPlan::new(replication_manifest.project, replication_manifest.hooks, packages);
            write_yaml_file(&replication_plan, "./source-wand/replication-plan.yaml")?;

            top_level.cleanup();
        },
        PackageOrigin::GoCache(_origin) => {},
    }

    Ok(())
}
