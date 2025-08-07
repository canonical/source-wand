use std::{fs::create_dir_all, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use source_wand_common::{project_manipulator::{local_project_manipulator::LocalProjectManipulator, project_manipulator::ProjectManipulator}, utils::read_yaml_file::read_yaml_file};
use source_wand_replication::model::{package_destination::PackageDestination, package_origin::PackageOrigin, replication_plan::ReplicationPlan};

#[derive(Debug, Parser)]
pub struct ReplicationApplyArgs;

pub fn replicate_apply_command(_args: &ReplicationApplyArgs) -> Result<()> {
    let replication_plan: ReplicationPlan = read_yaml_file("source-wand/replication-plan.yaml")?;
    
    for package in replication_plan.packages {
        if let PackageOrigin::GoCache(origin) = package.origin {
            let PackageDestination::Git(destination) = package.destination;
            let dependency_directory: PathBuf = PathBuf::from(format!("./source-wand/dependencies/{}/{}", origin.name, origin.version));
            create_dir_all(&dependency_directory)?;

            let sh: LocalProjectManipulator = LocalProjectManipulator::new(dependency_directory, true);

            println!("Fetching {} ({}) from the local Go Cache", origin.name, origin.version);
            sh.run_shell(format!("cp -r {}/* .", origin.path))?;

            let ls_remote: Result<String> = sh.run_shell(format!("git ls-remote --exit-code --heads {} {}", destination.git, destination.reference));

            if ls_remote.is_ok() {
                println!("{} ({}) already exists on remote, skipping", origin.name, origin.version);
                continue;
            }

            println!("Pushing {} ({}) source code to remote repository", origin.name, origin.version);
            sh.run_shell("git init".to_string())?;
            sh.run_shell(format!("git remote add origin {}", destination.git))?;
            sh.run_shell(format!("git checkout --orphan {}", destination.reference))?;
            sh.run_shell("git add .".to_string())?;
            sh.run_shell("git commit -m 'Replicate source code'".to_string())?;
            sh.run_shell(format!("git push -u origin {}", destination.reference))?;
        }
    }

    Ok(())
}
