use std::sync::Arc;

use crate::{model::{package_destination::PackageDestination, package_origin::PackageOrigin, replication_plan::ReplicationPlan}, plan::{environment::Environment, transformation_node::{NodeId, TransformationNode}, transformations::{git::{git_init::GitInit, git_push::GitPush}, golang::fetch_source::GolangFetchSource, sourcecraft::initialize::SourcecraftInitialize}}};

impl ReplicationPlan {
    pub fn to_execution_graph(&self) -> Vec<Arc<TransformationNode>> {
        let mut execution_graph: Vec<Arc<TransformationNode>> = Vec::new();

        let mut id: NodeId = 0;
        for package in &self.packages {
            if let PackageOrigin::GoCache(origin) = &package.origin {
                let PackageDestination::Git(destination) = &package.destination;

                let environment: Environment = Environment::new(&origin.name, &origin.version);
                let workdesk: String = format!("{} ({}-24.04/edge)", environment.name, environment.version_retrocompatible);

                let fetch: TransformationNode = TransformationNode {
                    id,
                    workdesk: workdesk.clone(),
                    transformation: Arc::new(GolangFetchSource::new(origin.path.clone())),
                    dependencies: vec![],
                    dependents: vec![id + 1]
                };
    
                let init: TransformationNode = TransformationNode {
                    id: id + 1,
                    workdesk: workdesk.clone(),
                    transformation: Arc::new(GitInit::new(
                        destination.git.clone(),
                        destination.reference.clone(),
                        if let Some(config) = &self.config {
                            config.git_identity.clone()
                        }
                        else {
                            None
                        },
                    )),
                    dependencies: vec![id],
                    dependents: vec![id + 2]
                };
    
                let push: TransformationNode = TransformationNode {
                    id: id + 2,
                    workdesk: workdesk.clone(),
                    transformation: Arc::new(GitPush::new(
                        destination.git.clone(),
                        destination.reference.clone(),
                    )),
                    dependencies: vec![id + 1],
                    dependents: vec![id + 3]
                };

                let initialize_sourcecraft: TransformationNode = TransformationNode {
                    id: id + 3,
                    workdesk,
                    transformation: Arc::new(SourcecraftInitialize::new(
                        environment.name.clone(),
                        format!("{}-24.04", environment.version_retrocompatible.clone()),
                        "ubuntu@24.04".to_string(),
                        vec!["amd64".to_string()],
                        package.dependencies.clone(),
                        package.is_library,
                    )),
                    dependencies: vec![id + 2],
                    dependents: vec![]
                };

                execution_graph.push(Arc::new(fetch));
                execution_graph.push(Arc::new(init));
                execution_graph.push(Arc::new(push));
                execution_graph.push(Arc::new(initialize_sourcecraft));

                id += 4;
            }
        }

        execution_graph
    }
}
