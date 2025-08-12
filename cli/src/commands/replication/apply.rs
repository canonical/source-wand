use std::{fs::{create_dir_all, remove_dir_all}, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use colorize::AnsiColor;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use source_wand_common::{
    project_manipulator::{
        local_project_manipulator::LocalProjectManipulator,
        project_manipulator::ProjectManipulator
    }
};
use source_wand_replication::model::{package::Package, package_destination::PackageDestination, package_origin::PackageOrigin, replication_plan::ReplicationPlan};
use uuid::Uuid;

use crate::commands::replication::plan::plan_replication;

#[derive(Debug, Parser)]
pub struct ReplicationApplyArgs;

fn replicate_package(package: Package) -> Result<()> {
    if let PackageOrigin::GoCache(origin) = package.origin {
        let PackageDestination::Git(destination) = package.destination;

        let uuid: Uuid = Uuid::new_v4();
        let dependency_directory: PathBuf = PathBuf::from(format!("./source-wand/{}", uuid));

        create_dir_all(&dependency_directory)?;

        let sh: LocalProjectManipulator = LocalProjectManipulator::new(dependency_directory, true);

        let ls_remote: Result<String> = sh.run_shell(format!("git ls-remote --exit-code --heads {} {}", destination.git, destination.reference));

        if ls_remote.is_ok() {
            println!("{}", format!("[skipped] {} ({}), already exists on remote", origin.name, origin.version).yellow());
            return Ok(());
        }

        sh.run_shell(format!("cp -r {}/* .", origin.path))?;
        sh.run_shell("git init".to_string())?;
        sh.run_shell(format!("git remote add origin {}", destination.git))?;
        sh.run_shell(format!("git checkout --orphan {}", destination.reference))?;
        sh.run_shell("git add .".to_string())?;
        sh.run_shell("git commit -m 'Replicate source code'".to_string())?;
        sh.run_shell(format!("git push -u origin {}", destination.reference))?;

        println!("{}", format!("[replicated] {} ({})", origin.name, origin.version).green());

        sh.cleanup();
    }

    Ok(())
}

pub fn replicate_apply_command(_args: &ReplicationApplyArgs) -> Result<()> {
    let replication_plan: ReplicationPlan = plan_replication()?;

    let results: Vec<Result<()>> = replication_plan
        .packages
        .into_par_iter()
        .map(replicate_package)
        .collect();

    remove_dir_all(PathBuf::from("./source-wand")).ok();

    for result in results {
        result?;
    }

    Ok(())
}
