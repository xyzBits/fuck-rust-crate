use std::collections::BTreeMap;
use std::path::Path;
use std::sync::{Arc, RwLock};
use openraft::{LogId, SnapshotMeta, StoredMembership};
use rocksdb::DB;
use serde::{Deserialize, Serialize};
use crate::{NodeId, TypeConfig};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Request {
    Set {key: String, value: String},
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Response {
    pub value: Option<String>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoredSnapshot {
    pub meta: SnapshotMeta<TypeConfig>,

    pub data: Vec<u8>,
}

pub struct StateMachineStore {
    pub data: StateMachineData,

    snapshot_idx: u64,

    db: Arc<DB>,
}

pub struct StateMachineData {
    pub last_applied_id: Option<LogId<NodeId>>,

    pub last_membership: StoredMembership<TypeConfig, NodeId>,

    pub kvs: Arc<RwLock<BTreeMap<String, String>>>,
}

#[derive(Clone, Debug)]
pub struct LogStore {
    db: Arc<DB>,
}



pub(crate) async fn new_storage<P: AsRef<Path>>(pd_path: P) -> (LogStore, StateMachineStore) {

    todo!()
}
