use serde::{Serialize, Deserialize};

use crate::model::{
    automatic_replication_manifest::AutomaticReplicationManifest,
    manual_replication_manifest::ManualReplicationManifest
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplicationManifest {
    Automatic(AutomaticReplicationManifest),
    Manual(ManualReplicationManifest),
}
