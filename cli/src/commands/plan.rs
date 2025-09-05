use std::fs;
use std::{fs::File, path::PathBuf};
use std::io::{Write, BufWriter};
use anyhow::{bail, Result};
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
pub struct PlanArgs {
    #[arg(long)]
    pub export_csv: Option<PathBuf>,
}

pub fn replicate_plan_command(args: &PlanArgs) -> Result<()> {
    let export_path: Option<PathBuf> = if let Some(output) = &args.export_csv {
        if let Some(extension) = output.extension() {
            if extension != "csv" {
                bail!("CSV export path extension must be .csv")
            }
            else {
                Some(output.clone())
            }
        }
        else {
            Some(output.with_extension(".csv"))
        }
    }
    else {
        None
    };

    let plan: ReplicationPlan = plan_replication()?;

    println!(
        "{} {} packages were identified as required to build the project",
        "[plan]".green(),
        format!("{}", plan.packages.len()).blue(),
    );

    for package in &plan.packages {
        let (
                name,
                version,
                source,
        ) = match &package.origin {
            PackageOrigin::Git(origin) => {
                let name: SanitizedName = SanitizedName::new(&origin.git);
                let version: SemanticVersion = SemanticVersion::new(&origin.reference);

                (name, version, origin.git.clone())
            },
            PackageOrigin::GoCache(origin) => {
                let name: SanitizedName = SanitizedName::new(&origin.name);
                let version: SemanticVersion = SemanticVersion::new(&origin.version);

                (name, version, origin.upstream.clone())
            },
        };

        println!(
            "\n{} package: {}",
            "[plan]".green(),
            name.sanitized.clone().italic(),
        );

        println!(
            "{} version: {}",
            "[plan]".green(),
            version.raw.clone().italic(),
        );

        println!(
            "{} channel: {}",
            "[plan]".green(),
            format!("{}-24.04/edge", version.retrocompatible).to_string().italic(),
        );

        println!(
            "{} source: {}",
            "[plan]".green(),
            source.italic(),
        );
    }

    if let Some(export_path) = export_path {
        let file: File = File::create(&export_path)?;
        let mut writer: BufWriter<File> = BufWriter::new(file);

        writeln!(writer, "package,version,track,source")?;

        for package in &plan.packages {
            let (
                name,
                version,
                source,
            ) = match &package.origin {
                PackageOrigin::Git(origin) => {
                    let name: SanitizedName = SanitizedName::new(&origin.git);
                    let version: SemanticVersion = SemanticVersion::new(&origin.reference);

                    (name, version, origin.git.clone())
                },
                PackageOrigin::GoCache(origin) => {
                    let name: SanitizedName = SanitizedName::new(&origin.name);
                    let version: SemanticVersion = SemanticVersion::new(&origin.version);

                    (name, version, origin.upstream.clone())
                },
            };

            writeln!(
                writer,
                "{},{},{},{}",
                name.sanitized.clone(),
                version.raw.clone(),
                format!("{}-24.04", version.retrocompatible),
                source,
            )?;
        }

        println!(
            "\n{} {} {}",
            "[execute]".green(),
            "exported analysis to CSV file".blue(),
            fs::canonicalize(export_path)?.as_os_str().to_str().unwrap().to_string().italic(),
        );
    }

    Ok(())
}
