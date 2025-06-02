use std::{fs::create_dir_all, path::PathBuf};

use anyhow::{bail, Result};
use source_wand_common::{
    project::Project,
    project_manipulator::{
        local_project_manipulator::LocalProjectManipulator,
        project_manipulator::ProjectManipulator
    }
};

use super::onboarding_source::OnboardingSource;

pub fn fetch_source(project: &Project) -> Result<OnboardingSource> {
    let project_directory: PathBuf = PathBuf::from(
        format!(
            "./packages/{}-{}/repository/",
            project.name.replace("/", "-"),
            project.version.replace("/", "-"),
        )
    );
    create_dir_all(&project_directory)?;

    let project_manipulator: LocalProjectManipulator = LocalProjectManipulator::new(project_directory);

    let tags_raw: String = project_manipulator.run_shell(format!("git ls-remote --tags {}", project.repository))?;
    let tags: Vec<&str> = tags_raw.lines()
        .into_iter()
        .filter_map(|tag| tag.split("\t").last())
        .collect();

    let branches_raw: String = project_manipulator.run_shell(format!("git ls-remote --branches {}", project.repository))?;
    let branches: Vec<&str> = branches_raw.lines()
        .skip(1)
        .filter_map(|branch| branch.split("\t").last())
        .collect();

    let checkout: String =
        if let Some(tag) = tags.iter().find(|tag| tag.contains(&project.version)) {
            tag.to_string()
        }
        else if let Some(branch) = branches.iter().find(|branch| branch.contains(&project.version)) {
            branch.to_string()
        }
        else {
            project.version.to_string() 
            //bail!("No tag or branch matches the package version")
        };

    Ok(OnboardingSource::git(project.repository.clone(), checkout))
}
