use anyhow::Result;
use source_wand_common::{project::Project, utils::{
    read_yaml_file::read_yaml_file,
    write_yaml_file::write_yaml_file
}};
use source_wand_dependency_analysis::dependency_tree_node::DependencyTreeNode;

use crate::plan::{
    onboarding_plan::OnboardingPlan,
    onboarding_source::OnboardingSource
};

pub fn plan_onboarding() -> Result<usize> {
    let dependency_tree: DependencyTreeNode = read_yaml_file("dependencies.yaml")?;
    let mut nb_manual_requests: usize = 0;

    println!(" > Generating onboarding plans for each dependency");
    for dependency in dependency_tree.flatten().dependencies {
        println!("  > {} ({})", dependency.name, dependency.version);
        if let Ok(plan) = generate_onboarding_plan(&dependency) {
            write_yaml_file(
                &plan,
                format!(
                    "packages/{}-{}/onboard.yaml",
                    dependency.name.replace("/", "-"),
                    dependency.version.replace("/", "-"),
                ).as_str()
            )?;
        }
        else {
            let plan: OnboardingPlan = OnboardingPlan::new(
                dependency.name.clone(),
                dependency.version.clone(),
                dependency.license,
                OnboardingSource::to_complete(),
                format!("{}/edge", dependency.version),
                Vec::new()
            );
            write_yaml_file(
                &plan,
                format!(
                    "to-complete/{}-{}.yaml",
                    dependency.name.replace("/", "-"),
                    dependency.version.replace("/", "-"),
                ).as_str()
            )?;

            nb_manual_requests += 1;
        }
    }

    Ok(nb_manual_requests)
}

fn generate_onboarding_plan(dependency: &Project) -> Result<OnboardingPlan> {
    let plan: OnboardingPlan = OnboardingPlan::new(
        dependency.name.clone(),
        dependency.version.clone(),
        dependency.license.clone(),
        OnboardingSource::git(dependency.repository.clone(), dependency.version.clone()),
        format!("{}/edge", dependency.version),
        Vec::new(),
    );
    Ok(plan)
}
