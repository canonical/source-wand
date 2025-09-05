use anyhow::Result;
use clap::Parser;
use colorize::AnsiColor;
use source_wand_common::identity::{
    sanitized_name::SanitizedName,
    semantic_version::SemanticVersion
};
use source_wand_replication::{
    model::{
        package_origin::PackageOrigin,
        replication_plan::ReplicationPlan
    },
    plan::planner::plan_replication
};

#[derive(Debug, Parser)]
pub struct PlanArgs;

pub fn replicate_plan_command(_args: &PlanArgs) -> Result<()> {
    let plan: ReplicationPlan = plan_replication()?;

    println!(
        "{} {} packages were identified as required to build the project",
        "[plan]".green(),
        format!("{}", plan.packages.len()).blue(),
    );

    for package in plan.packages {
        println!();
        let (sanitized_name, semantic_version) = match package.origin {
            PackageOrigin::Git(origin) => {
                let sanitized_name: SanitizedName = SanitizedName::new(&origin.git);
                let semantic_version: SemanticVersion = SemanticVersion::new(&origin.reference);

                (sanitized_name, semantic_version)
            },
            PackageOrigin::GoCache(origin) => {
                let sanitized_name: SanitizedName = SanitizedName::new(&origin.name);
                let semantic_version: SemanticVersion = SemanticVersion::new(&origin.version);

                (sanitized_name, semantic_version)
            },
        };

        println!(
            "{} package: {}",
            "[plan]".green(),
            sanitized_name.value.clone().italic(),
        );

        println!(
            "{} version: {}",
            "[plan]".green(),
            semantic_version.raw.clone().italic(),
        );

        println!(
            "{} channel: {}",
            "[plan]".green(),
            format!("{}-24.04/edge", semantic_version.retrocompatible).to_string().italic(),
        );
    }

    Ok(())
}
