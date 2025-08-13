use anyhow::Result;
use clap::Parser;
use source_wand_common::utils::write_yaml_file::write_yaml_file;
use source_wand_replication::model::{
    package_destination_git::PackageDestinationGit,
    package_origin_git::PackageOriginGit,
    replication_manifest::ReplicationManifest,
    replication_project::ReplicationProject
};

#[derive(Debug, Parser)]
pub struct ReplicationInitArgs;

pub fn replicate_init_command(_args: &ReplicationInitArgs) -> Result<()> {
    let replication_manifest: ReplicationManifest = ReplicationManifest::new(
        ReplicationProject::new(
            PackageOriginGit::new("<url to your project's repository>".to_string(), "<reference to checkout>".to_string()),
            PackageDestinationGit::new("<where to replicate your project>".to_string(), "<reference to push>".to_string()),
        ),
    );

    write_yaml_file(&replication_manifest, "replication.yaml")?;

    println!("Replication project initialized.");
    Ok(())
}
