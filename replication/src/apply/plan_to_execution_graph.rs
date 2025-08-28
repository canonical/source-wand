use std::sync::Arc;

use crate::{
    model::{
        package_destination::PackageDestination,
        package_origin::PackageOrigin,
        replication_plan::ReplicationPlan
    },
    plan::{
        environment::Environment,
        transformation_node::{
            NodeId,
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
        let mut execution_graph: Vec<Arc<TransformationNode>> = Vec::new();

        let mut id: NodeId = 0;
        for package in &self.packages {
            if let PackageOrigin::GoCache(origin) = &package.origin {
                let PackageDestination::Git(destination) = &package.destination;

                let environment: Environment = Environment::new(&origin.name, &origin.version);
                let workdesk: String = format!("{} ({}-24.04/edge)", environment.name, environment.version_retrocompatible);

                let initialize_project: TransformationNode = TransformationNode {
                    id: id,
                    workdesk: workdesk.clone(),
                    transformation: Arc::new(
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
                    ),
                    dependencies: vec![],
                };
    
                let push_code: TransformationNode = TransformationNode {
                    id: id + 1,
                    workdesk: workdesk.clone(),
                    transformation: Arc::new(GitPush::new(
                        destination.reference.clone(),
                        "Replicate source code".to_string(),
                    )),
                    dependencies: vec![id],
                };

                let initialize_sourcecraft: TransformationNode = TransformationNode {
                    id: id + 2,
                    workdesk: workdesk.clone(),
                    transformation: Arc::new(SourcecraftInitialize::new(
                        environment.name.clone(),
                        format!("{}-24.04", environment.version_retrocompatible.clone()),
                        "ubuntu@24.04".to_string(),
                        vec!["amd64".to_string()],
                        package.dependencies.clone(),
                        package.is_library,
                    )),
                    dependencies: vec![id + 1],
                };

                let push_sourcecraft_metadata: TransformationNode = TransformationNode {
                    id: id + 3,
                    workdesk: workdesk,
                    transformation: Arc::new(GitPush::new(
                        destination.reference.clone(),
                        "Initialize sourcecraft".to_string(),
                    )),
                    dependencies: vec![id + 2],
                };

                execution_graph.push(Arc::new(initialize_project));
                execution_graph.push(Arc::new(push_code));
                execution_graph.push(Arc::new(initialize_sourcecraft));
                execution_graph.push(Arc::new(push_sourcecraft_metadata));

                id += 4;
            }
        }

        execution_graph
    }
}
