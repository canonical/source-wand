use std::{env, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use source_wand_common::utils::write_yaml_file::write_yaml_file;
use source_wand_replication::model::{hooks::Hooks, package_destination_git::PackageDestinationGit, package_origin_git::PackageOriginGit, replication_manifest::ReplicationManifest};

#[derive(Debug, Parser)]
pub struct ReplicateInitArgs;

pub fn replicate_init_command(_args: &ReplicateInitArgs) -> Result<()> {
    let working_directory: PathBuf = env::current_dir()?;
    let project_name = working_directory
        .file_name()
        .and_then(|os_string| os_string.to_str())
        .ok_or_else(|| anyhow::anyhow!("unknown"))?
        .to_string();

    let replication_manifest: ReplicationManifest = ReplicationManifest::new(
        project_name,
        Some(Hooks { before_all: None, before_each: None, after_each: None, after_all: None }),
        PackageOriginGit::new("<url to your project's repository>".to_string(), "<reference to checkout>".to_string()),
        PackageDestinationGit::new("<where to replicate your project>".to_string(), "<reference to push>".to_string()),
    );

    write_yaml_file(&replication_manifest, "replication.yaml")?;

    println!("Replication project initialized.");
    Ok(())
}
