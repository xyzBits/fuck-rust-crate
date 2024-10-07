use std::fmt::Display;
use std::io::Cursor;
use std::sync::Arc;
use openraft::Config;
use serde::{Deserialize, Serialize};
use tracing::info;
use crate::store::{Request, Response};

pub mod app;
pub mod client;
pub mod network;
pub mod store;

pub type NodeId = u64;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Node {
    pub rpc_addr: String,
    pub api_addr: String,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node {{ rpc_addr: {}, api_addr: {} }}", self.rpc_addr, self.api_addr)
    }
}


pub type SnapshotData = Cursor<Vec<u8>>;



openraft::declare_raft_types!(
    pub TypeConfig:
    D =  Request,
    R = Response,
    Node = Node,
);


pub mod typ {
    use openraft::error::Infallible;
    use crate::TypeConfig;

    pub type Entry = openraft::Entry<TypeConfig>;

    pub type RaftError<E = Infallible> = openraft::error::RaftError<TypeConfig, E>;

    pub type RPCError<E = Infallible> = openraft::error::RPCError<TypeConfig, RaftError<E>>;


    // pub type ClientWriteError = openraft::error::ClientWriteError<TypeConfig, openraft::Node>;

    // pub type CheckIsLeaderError = openraft::error::CheckIsLeaderError<TypeConfig>;




}



pub async fn start_example_raft_node<P>(
    node_id: NodeId,
    dir: P,
    http_addr: String,
    rpc_addr: String,
) -> std::io::Result<()>
where
    P: AsRef<std::path::Path>,
{
    let config = Config {
        heartbeat_interval: 250,
        election_timeout_min: 299,
        cluster_name: "raft-kv".to_string(),
        ..Default::default()
    };

    let config = Arc::new(config.validate().unwrap());

    info!("raft config: {:?}", config);






    Ok(())
}