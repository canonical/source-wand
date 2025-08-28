use std::sync::Arc;

use crate::{
    model::{
        package_destination::PackageDestination,
        package_origin::PackageOrigin,
        replication_plan::ReplicationPlan
    },
    plan::{
        environment::Environment,
        execution_graph_builder::{
            ExecutionGraphBuilder,
            RcExecutionNodeBuilder
        },
        transformation_node::{
            TransformationNode
        },
        transformations::{
            git::{
                git_init::GitInit,
                git_push::GitPush
            },
            golang::fetch_source::GolangFetchSource,
            initialize_project::InitializeProject,
            sourcecraft::initialize::SourcecraftInitialize
        }
    }
};

impl ReplicationPlan {
    pub fn to_execution_graph(&self) -> Vec<Arc<TransformationNode>> {
        let mut execution_graph_builder: ExecutionGraphBuilder = ExecutionGraphBuilder::new();

        for package in &self.packages {
            if let PackageOrigin::GoCache(origin) = &package.origin {
                let PackageDestination::Git(destination) = &package.destination;

                let environment: Environment = Environment::new(&origin.name, &origin.version);
                let workdesk: String = format!("{} ({}-24.04/edge)", environment.name, environment.version_retrocompatible);

                let initialize_project: RcExecutionNodeBuilder = execution_graph_builder.create_node(
                    workdesk.clone(),
                    Arc::new(
                        InitializeProject::new(
                            GitInit::new(
                                destination.git.clone(),
                                destination.reference.clone(),
                                if let Some(config) = &self.config {
                                    config.git_identity.clone()
                                }
                                else {
                                    None
                                },
                            ),
                            GolangFetchSource::new(origin.path.clone()),
                        )
                    )
                );

                let push_code: RcExecutionNodeBuilder = execution_graph_builder.create_node(
                    workdesk.clone(),
                    Arc::new(
                        GitPush::new(
                            destination.reference.clone(),
                            "Replicate source code".to_string(),
                        )
                    )
                );

                let initialize_sourcecraft: RcExecutionNodeBuilder = execution_graph_builder.create_node(
                    workdesk.clone(),
                    Arc::new(
                        SourcecraftInitialize::new(
                            environment.name.clone(),
                            format!("{}-24.04", environment.version_retrocompatible.clone()),
                            "ubuntu@24.04".to_string(),
                            vec!["amd64".to_string()],
                            package.dependencies.clone(),
                            package.is_library,
                        )
                    )
                );

                let push_sourcecraft_metadata: RcExecutionNodeBuilder = execution_graph_builder.create_node(
                    workdesk.clone(),
                    Arc::new(
                        GitPush::new(
                            destination.reference.clone(),
                            "Initialize sourcecraft".to_string(),
                        )
                    )
                );

                push_code.borrow_mut().depends_on(&initialize_project);
                initialize_sourcecraft.borrow_mut().depends_on(&push_code);
                push_sourcecraft_metadata.borrow_mut().depends_on(&initialize_sourcecraft);
            }
        }

        execution_graph_builder.build()
    }
}
