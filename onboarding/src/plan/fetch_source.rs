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
    project_manipulator.run_shell(format!("git clone {} .", project.repository))?;

    let tags_raw: String = project_manipulator.run_shell("git tag".to_string())?;
    let tags: Vec<&str> = tags_raw.lines().into_iter().collect();

    let branches_raw: String = project_manipulator.run_shell("git branch -a".to_string())?;
    let branches: Vec<&str> = branches_raw.lines().skip(1).map(|branch| branch.trim()).collect();

    let checkout: String = if tags.contains(&project.version.as_str()) {
        project.version.clone()
    }
    else if let Some(branch) = branches.iter().find(|branch| branch.ends_with(&project.version)) {
        branch.to_string()
    }
    else {
        bail!("No tag or branch matches the package version")
    };

    Ok(OnboardingSource::git(project.repository.clone(), checkout))
}
