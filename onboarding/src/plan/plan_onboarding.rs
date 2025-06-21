use std::sync::{atomic::{AtomicUsize, Ordering}, Arc};

use anyhow::Result;
use colorize::AnsiColor;
use futures::future::try_join_all;
use source_wand_common::{
    project::Project,
    utils::{
        read_yaml_file::read_yaml_file,
        write_text_file::write_text_file,
        write_yaml_file::write_yaml_file
    }
};
use source_wand_dependency_analysis::{
    // dependency_tree_node::DependencyTreeNode,
    unique_dependencies_list::UniqueDependenciesList
};
use tokio::runtime::Runtime;

use crate::plan::{
    onboarding_plan::OnboardingPlan,
    onboarding_source::OnboardingSource
};

use super::fetch_source::fetch_source;

pub fn plan_onboarding() -> Result<usize> {
    let runtime: Runtime = tokio::runtime::Runtime::new()?;
    runtime.block_on(plan_onboarding_async())
}

async fn plan_onboarding_async() -> Result<usize> {
    // let dependency_tree: DependencyTreeNode = read_yaml_file("dependencies.yaml")?;
    let build_requirements: UniqueDependenciesList = read_yaml_file("build-requirements.yaml")?;

    let nb_manual_requests: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));

    println!(" > Generating onboarding plans for each dependency");
    let tasks = build_requirements
        .dependencies
        .into_iter()
        .map(|dependency| {
            let nb_manual_requests: Arc<AtomicUsize> = Arc::clone(&nb_manual_requests);
            tokio::spawn(async move {
                plan_dependency_onboarding(dependency, nb_manual_requests).await
            })
        })
        .collect::<Vec<_>>();

    try_join_all(tasks).await?;

    Ok(nb_manual_requests.load(Ordering::Relaxed))
}

async fn plan_dependency_onboarding(dependency: Project, nb_manual_requests: Arc<AtomicUsize>) -> Result<()> {
    match generate_onboarding_plan(&dependency) {
        Ok(plan) => {
            println!("{}", format!("  ✓ {} ({})", dependency.name, dependency.version).green());
            write_yaml_file(
                &plan,
                format!(
                    "packages/{}-{}/source-wand.yaml",
                    dependency.name.replace("/", "-"),
                    dependency.version.replace("/", "-"),
                ).as_str()
            )?;
        },
        Err(e) => {
            println!("{}", format!("  × {} ({})", dependency.name, dependency.version).red());
            println!("{}", format!("  × > {}", e).yellow());

            let plan: OnboardingPlan = OnboardingPlan::new(
                dependency.name.clone(),
                dependency.version.clone(),
                dependency.license,
                OnboardingSource::to_complete(),
                format!("{}/edge", dependency.version),
                Vec::new()
            );
            write_text_file(
                &e.to_string(),
                format!(
                    "to-complete/{}-{}/logs.yaml",
                    dependency.name.replace("/", "-"),
                    dependency.version.replace("/", "-"),
                ).as_str()
            )?;
            write_yaml_file(
                &plan,
                format!(
                    "to-complete/{}-{}/source-wand.yaml",
                    dependency.name.replace("/", "-"),
                    dependency.version.replace("/", "-"),
                ).as_str()
            )?;

            nb_manual_requests.fetch_add(1, Ordering::Relaxed);
        },
    }

    Ok(())
}

fn generate_onboarding_plan(dependency: &Project) -> Result<OnboardingPlan> {
    let plan: OnboardingPlan = OnboardingPlan::new(
        dependency.name.clone(),
        dependency.version.clone(),
        dependency.license.clone(),
        fetch_source(&dependency)?,
        format!("{}/edge", dependency.version),
        Vec::new(),
    );
    Ok(plan)
}
