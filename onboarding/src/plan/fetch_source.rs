use std::{fs::create_dir_all, path::PathBuf};

use anyhow::{bail, Result};
use regex::Regex;
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

    let commit_hash_regex: Regex = Regex::new(r"^v\d+\.\d+\.\d+-\d{8}\d{6}-([a-f0-9]+)$")?;
    let potential_commit_hash: Option<String> = commit_hash_regex.captures(project.version.as_str())
        .and_then(|captures| captures.get(1).map(|part| part.as_str().to_string()));

    let checkout: String =
        if let Some(tag) = tags.iter().find(|tag| tag.contains(&project.version)) {
            tag.to_string()
        }
        else if let Some(branch) = branches.iter().find(|branch| branch.contains(&project.version)) {
            branch.to_string()
        }
        else if let Some(potential_commit_hash) = potential_commit_hash {
            project_manipulator.run_shell(format!("git clone --no-checkout {} .", project.repository))?;
            project_manipulator.run_shell(format!("git checkout {}", potential_commit_hash))?;
            potential_commit_hash
        }
        else {
            bail!("No tag, branch or commit matches the package version")
        };

    Ok(OnboardingSource::git(project.repository.clone(), checkout))
}
