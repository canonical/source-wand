use std::sync::Arc;

use crate::{model::{package_destination::PackageDestination, package_origin::PackageOrigin, replication_plan::ReplicationPlan}, plan::{transformation_node::{NodeId, TransformationNode}, transformations::{git::{git_init::GitInit, git_push::GitPush}, golang::fetch_source::GolangFetchSource}}};

impl ReplicationPlan {
    pub fn to_execution_graph(&self) -> Vec<Arc<TransformationNode>> {
        let mut execution_graph: Vec<Arc<TransformationNode>> = Vec::new();

        let mut id: NodeId = 0;
        for package in &self.packages {
            if let PackageOrigin::GoCache(origin) = &package.origin {
                let PackageDestination::Git(destination) = &package.destination;

                let workdesk: String = format!("{} ({})", origin.name, origin.version);

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
                    transformation: Arc::new(GitInit::new(destination.git.clone(), destination.reference.clone())),
                    dependencies: vec![id],
                    dependents: vec![id + 2]
                };
    
                let push: TransformationNode = TransformationNode {
                    id: id + 2,
                    workdesk,
                    transformation: Arc::new(GitPush::new(destination.git.clone(), destination.reference.clone())),
                    dependencies: vec![id + 1],
                    dependents: vec![]
                };

                execution_graph.push(Arc::new(fetch));
                execution_graph.push(Arc::new(init));
                execution_graph.push(Arc::new(push));

                id += 3;
            }
        }

        execution_graph
    }
}

// use std::{collections::HashMap, sync::Arc};

// use crate::{model::{package::Package, replication_plan::ReplicationPlan}, plan::{transformation_node::{NodeId, TransformationNode}, transformations::{git::{git_init::GitInit, git_push::GitPush}, golang::fetch_source::GolangFetchSource}}};


// impl ReplicationPlan {
//     pub fn to_execution_graph(&self) -> Vec<Arc<TransformationNode>> {
//         let mut nodes: Vec<Arc<TransformationNode>> = Vec::new();
//         let mut node_id_counter = 0;

//         let mut package_nodes: HashMap<String, (NodeId, NodeId)> = HashMap::new();

//         for package in &self.packages {
//             let golang_fetch = Arc::new(GolangFetchSource::new(match &package.origin {
//                 crate::model::package_origin::PackageOrigin::GoCache(origin) => origin.path.clone(),
//                 crate::model::package_origin::PackageOrigin::Git(origin) => origin.git.clone(),
//             }));

//             let golang_fetch_node_id = node_id_counter;
//             node_id_counter += 1;

//             let golang_fetch_node = TransformationNode {
//                 id: golang_fetch_node_id,
//                 transformation: golang_fetch,
//                 dependencies: Vec::new(),
//                 dependents: Vec::new(),
//             };

//             let (repo_url, reference) = match &package.destination {
//                 crate::model::package_destination::PackageDestination::Git(dest) => (&dest.git, &dest.reference),
//             };
//             let git_init = Arc::new(GitInit::new(repo_url.clone(), reference.clone()));

//             let git_init_node_id = node_id_counter;
//             node_id_counter += 1;

//             let git_init_node = TransformationNode {
//                 id: git_init_node_id,
//                 transformation: git_init,
//                 dependencies: vec![golang_fetch_node_id],
//                 dependents: Vec::new(),
//             };

//             let git_push = Arc::new(GitPush::new(repo_url.clone(), reference.clone()));

//             let git_push_node_id = node_id_counter;
//             node_id_counter += 1;

//             let git_push_node = TransformationNode {
//                 id: git_push_node_id,
//                 transformation: git_push,
//                 dependencies: vec![git_init_node_id],
//                 dependents: Vec::new(),
//             };

//             let mut golang_fetch_node = Arc::try_unwrap(golang_fetch_node.into())
//                 .unwrap_or_else(|arc| (*arc).clone());
//             golang_fetch_node.dependents.push(git_init_node_id);
//             let mut git_init_node = Arc::try_unwrap(git_init_node.into())
//                 .unwrap_or_else(|arc| (*arc).clone());
//             git_init_node.dependents.push(git_push_node_id);

//             nodes.push(Arc::new(golang_fetch_node));
//             nodes.push(Arc::new(git_init_node));
//             nodes.push(Arc::new(git_push_node));

//             package_nodes.insert(package.origin_key(), (golang_fetch_node_id, git_push_node_id));
//         }

//         let mut id_to_node = nodes.iter().map(|n| (n.id, Arc::clone(n))).collect::<HashMap<_, _>>();

//         for package in &self.packages {
//             let (pkg_golang_fetch_id, _) = package_nodes.get(&package.origin_key()).unwrap();

//             for dependency in &package.dependencies {
//                 if let Some((_, dep_git_push_id)) = package_nodes.get(&dependency.name) {
//                     let mut pkg_golang_fetch_node = Arc::try_unwrap(id_to_node[pkg_golang_fetch_id].clone())
//                         .unwrap_or_else(|arc| (*arc).clone());
//                     if !pkg_golang_fetch_node.dependencies.contains(dep_git_push_id) {
//                         pkg_golang_fetch_node.dependencies.push(*dep_git_push_id);
//                     }

//                     let mut dep_git_push_node = Arc::try_unwrap(id_to_node[dep_git_push_id].clone())
//                         .unwrap_or_else(|arc| (*arc).clone());
//                     if !dep_git_push_node.dependents.contains(pkg_golang_fetch_id) {
//                         dep_git_push_node.dependents.push(*pkg_golang_fetch_id);
//                     }

//                     id_to_node.insert(*pkg_golang_fetch_id, Arc::new(pkg_golang_fetch_node));
//                     id_to_node.insert(*dep_git_push_id, Arc::new(dep_git_push_node));
//                 }
//             }
//         }

//         id_to_node.values().cloned().collect()
//     }
// }

// impl Package {
//     fn origin_key(&self) -> String {
//         match &self.origin {
//             crate::model::package_origin::PackageOrigin::Git(origin) => origin.git.clone(),
//             crate::model::package_origin::PackageOrigin::GoCache(origin) => origin.name.clone(),
//         }
//     }
// }
