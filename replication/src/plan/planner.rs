use std::{
    collections::HashSet,
    fs::create_dir_all,
    path::PathBuf,
    sync::{
        Arc,
        Mutex,
        MutexGuard
    }
};

use anyhow::{bail, Result};
use colorize::AnsiColor;
use serde_json::Value;
use source_wand_common::{
    identity::{
        sanitized_name::SanitizedName,
        semantic_version::SemanticVersion
    },
    project_manipulator::{
        local_project_manipulator::LocalProjectManipulator,
        project_manipulator::ProjectManipulator
    },
    utils::read_yaml_file::read_yaml_file
};
use source_wand_dependency_analysis::{
    dependency_tree_generators::{dependency_tree_graph::Graph, go_dependency_tree_generator_deep_replication::parse_dependency}, dependency_tree_node::DependencyTreeNode, dependency_tree_request::DependencyTreeRequest, find_dependency_tree, 
};
use uuid::Uuid;

use crate::model::{
    dependency::Dependency, package::Package, package_destination::PackageDestination, package_destination_git::PackageDestinationGit, package_origin::PackageOrigin, package_origin_git::PackageOriginGit, package_origin_go_cache::PackageOriginGoCache, replication_manifest::ReplicationManifest, replication_plan::ReplicationPlan
};

pub fn plan_replication() -> Result<ReplicationPlan> {
    println!(
        "{} analyzing the origin project's dependency tree",
        "[plan]".green(),
    );

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

            let dependency_tree: Arc<Mutex<DependencyTreeNode>> = find_dependency_tree(
                DependencyTreeRequest::from_git_project(
                    origin.git.clone(),
                    Some(origin.reference.clone())
                )
            )?;

            let build_dependencies_whitelist: HashSet<(String, String)> = {
                let raw: serde_json::Value = serde_json::from_str(
                    top_level.run_shell(
                        "go mod edit -json".to_string()
                    )?.as_str()
                )?;

                if let Value::Object(module) = raw {
                    let module_name: &String = match module.get("Module") {
                        Some(Value::Object(module)) => {
                            match module.get("Path") {
                                Some(Value::String(module_name)) => {
                                    &module_name
                                        .replace("/", "-")
                                        .replace(".", "-")
                                },
                                _ => bail!("Dependencies whitelist does not contain Module -> Path field"),
                            }
                        },
                        _ => bail!("Dependencies whitelist does not contain Module field"),
                    };

                    let module_version: String = origin.reference
                        .split('/')
                        .last()
                        .unwrap_or_default()
                        .to_string();

                    let dependencies: Vec<(String, String)> = match module.get("Require") {
                        Some(Value::Array(dependencies)) => {
                            let mut dependencies: Vec<(String, String)> = dependencies.iter()
                                .map(
                                    |dependency| {
                                        match dependency {
                                            Value::Object(dependency) => {
                                                let name: &String = match dependency.get("Path") {
                                                    Some(Value::String(path)) => {
                                                        &path
                                                            .replace("/", "-")
                                                            .replace(".", "-")
                                                    },
                                                    _ => bail!("Dependencies whitelist Require[i].Path was not a String"),
                                                };
                                                let version: &String = match dependency.get("Version") {
                                                    Some(Value::String(version)) => version,
                                                    _ => bail!("Dependencies whitelist Require[i].Version was not a String"),
                                                };

                                                Ok((name.clone(), version.clone()))
                                            },
                                            _ => bail!("Dependencies whitelist Require[i] field was not an object"),
                                        }
                                    }
                                )
                                .collect::<Result<Vec<_>, anyhow::Error>>()?;

                            dependencies.push((module_name.clone(), module_version.clone()));

                            dependencies
                        },
                        _ => bail!("Dependencies whitelist does not contain Require field")
                    };

                    dependencies.into_iter().collect::<HashSet<(String, String)>>()
                }
                else {
                    bail!("Dependencies whitelist is not an object")
                }
            };

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
                            .replace("/", "-")
                            .replace(".", "-");

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

                        if !build_dependencies_whitelist.contains(&(name.clone(), version.clone())) {
                            continue;
                        }

                        let cache_path: String = build_dependency.get("Dir")
                            .unwrap_or(&Value::String(String::new()))
                            .as_str()
                            .unwrap_or_default()
                            .to_string();

                        let upstream: String = build_dependency.get("Path")
                            .unwrap()
                            .as_str()
                            .unwrap_or_default()
                            .to_string();

                        let name: SanitizedName = SanitizedName::new(&name);
                        let version: SemanticVersion = SemanticVersion::new(&version);

                        let PackageDestination::Git(package_destination) = &replication_manifest.destination_template;

                        let package_destination_url: String = name.apply(&package_destination.git);
                        let package_destination_url: String = version.apply(&package_destination_url);

                        let package_destination_reference: String = name.apply(&package_destination.reference);
                        let package_destination_reference: String = version.apply(&package_destination_reference);

                        let dependencies: Vec<Dependency> = find_dependencies_for_package(
                            dependency_tree.clone(),
                            &name.original,
                        );

                        let package: Package = Package::new(
                            PackageOriginGoCache::new(
                                name.sanitized.clone(),
                                version.raw.clone(),
                                cache_path,
                                upstream,
                            ),
                            PackageDestinationGit::new(
                                package_destination_url,
                                package_destination_reference,
                            ),
                            dependencies,
                            !origin.git.clone()
                                .replace("/", "-")
                                .replace(".", "-")
                                .ends_with(&name.sanitized)
                        );

                        packages.push(package);
                    }
                }
            }

            top_level.cleanup();

            let replication_plan: ReplicationPlan = ReplicationPlan::new(
                replication_manifest.project,
                replication_manifest.hooks,
                packages,
                replication_manifest.config,
            );

            Ok(replication_plan)
        },
        PackageOrigin::GoCache(_origin) => { todo!() },
    }
}

fn find_dependencies_for_package(root: Arc<Mutex<DependencyTreeNode>>, package_name: &str) -> Vec<Dependency> {
    let (name, dependencies) = {
        let node: MutexGuard<'_, DependencyTreeNode> = root.lock().unwrap();
        (
            node.project.name.replace("/", "-").replace(".", "-"),
            node.dependencies.clone(),
        )
    };

    if name == package_name {
        return dependencies
            .iter()
            .map(|dep| {
                let dep_guard: MutexGuard<'_, DependencyTreeNode> = dep.lock().unwrap();

                let name: SanitizedName = SanitizedName::new(&dep_guard.project.name);
                let version: SemanticVersion = SemanticVersion::new(&dep_guard.project.version);

                Dependency {
                    name: name.sanitized.replace("/", "-").replace(".", "-"),
                    version: format!("{}-24.04", version.retrocompatible),
                }
            })
            .collect();
    }

    for child in dependencies {
        let found: Vec<Dependency> = find_dependencies_for_package(child, package_name);
        if !found.is_empty() {
            return found;
        }
    }

    Vec::new()
}

// Andrew's Changes

pub fn replication_plan_andrew_go(
    url: & String,
    version: & String,
    project_root: & PathBuf,
    module_name: & String,
) -> Result<ReplicationPlan> {
    let replication_manifest: ReplicationManifest = read_yaml_file("replication.yaml")?;
    match replication_manifest.origin {
        PackageOrigin::Git(origin) => {
            let uuid: Uuid = Uuid::new_v4();
            let top_level_directory: PathBuf = PathBuf::from(format!("./source-wand/{}", uuid));
            create_dir_all(&top_level_directory)?;

            let graph: Arc<Graph<DependencyTreeNode>> = Arc::new(Graph::new());
            parse_dependency(&url, &version, &project_root, &module_name, Arc::clone(&graph));
            let build_dependencies = graph.get_node_list();


            let mut packages: Vec<Package> = Vec::new();


            // ### Step: Create the list of packages
            for build_dependency in build_dependencies {
                let node = graph.get_node(&build_dependency).unwrap();
                let name: String = node.project.name.as_str().replace("/", "-").replace(".", "-");
                let reference = match node.project.checkout.clone() {
                    Some(value) => value,
                    None => "".to_string(),
                };
                // For every key in the hashset, get the associated Node (Name + Version) => Put it into the list
                let dependencies: Vec<Dependency> = get_dependency_packages(Arc::clone(&graph), &build_dependency);
                
                let name: SanitizedName = SanitizedName::new(&name);
                let version: SemanticVersion = SemanticVersion::new(&reference);

                let PackageDestination::Git(package_destination) = &replication_manifest.destination_template;

                let package_destination_url: String = name.apply(&package_destination.git);
                let package_destination_url: String = version.apply(&package_destination_url);

                let package_destination_reference: String = name.apply(&package_destination.reference);
                let package_destination_reference: String = version.apply(&package_destination_reference);




                let package: Package = Package::new(
                    PackageOriginGit::new(
                        node.project.repository.clone(),
                        reference
                    ),
                    PackageDestinationGit::new(
                        package_destination_url,
                        package_destination_reference,
                    ),
                    dependencies,
                    !origin.git.clone().replace("/", "-").replace(".", "-").ends_with(&name.sanitized));
                packages.push(package);
            }

            // ### Final Step: Create the replication plan ###
            let replication_plan: ReplicationPlan = ReplicationPlan::new(
                replication_manifest.project,
                replication_manifest.hooks,
                packages,
                replication_manifest.config
            );
            Ok(replication_plan)
        },
        PackageOrigin::GoCache(_origin) => { todo!() },
    }
}

fn get_dependency_packages(graph: Arc<Graph<DependencyTreeNode>>, build_dependency: &str) -> Vec<Dependency> {
    let mut ret = Vec::new();
    let dependencies_hash = match graph.get_edges(&build_dependency) {
        Some(hash) => hash,
        None => {
            HashSet::new()
        }
    };

    dependencies_hash.iter().for_each(|dep| {
        let node = graph.get_node(&dep).unwrap();
        ret.push(Dependency {
            name: node.project.name,
            version: node.project.version
        })
    });
    ret
}

