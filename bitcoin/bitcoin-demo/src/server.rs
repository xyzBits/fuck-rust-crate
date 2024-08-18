use std::collections::{HashMap, HashSet};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use crate::block::Block;
use crate::errors::Result;
use crate::transaction::Transaction;
use crate::utxoset::UTXOSet;

#[derive(Serialize, Deserialize, Debug, Clone)]
enum Message {
    Addr(Vec<String>),
    Version(VersionMsg),
    Tx(TxMsg),
    GetData(GetDataMsg),
    GetBlock(GetBlockMsg),
    Inv(InvMsg),
    Block(BlockMsg),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct BlockMsg {
    addr_from: String,
    block: Block,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GetBlockMsg {
    addr_from: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GetDataMsg {
    addr_from: String,
    kine: String,
    id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct InvMsg {
    addr_from: String,
    kine: String,
    items: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TxMsg {
    addr_from: String,
    transaction: Transaction,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct VersionMsg {
    addr_from: String,
    version: i32,
    best_height: i32,
}

pub struct Server {
    node_address: String,
    mining_address: String,
    inner: Arc<Mutex<ServerInner>>,
}

struct ServerInner {
    known_nodes: HashSet<String>,
    utxo: UTXOSet,
    blocks_in_transit: Vec<String>,
    memory_pool: HashMap<String, Transaction>,
}

const KNOWN_NODE1: &str = "localhost:3000";
const CMD_LEN: usize = 12;
const VERSION: i32 = 1;

impl Server {
    pub fn new(port: &str, miner_address: &str, utxoset: UTXOSet) -> Result<Server> {
        todo!()
    }

    pub fn start_server(&self) -> Result<()> {
        todo!()
    }

    pub fn send_transaction(tx: &Transaction, utxoset: UTXOSet) -> Result<()> {
        todo!()
    }

    fn remove_node(&self, addr: &str) {
        todo!()
    }

    fn add_nodes(&self, addr: &str) {
        todo!()
    }

    fn get_known_nodes(&self) -> HashSet<String> {
        todo!()
    }

    fn node_is_known(&self, addr: &str) -> bool {
        todo!()
    }

    fn replace_in_transit(&self, hashes: Vec<String>) {
        todo!()
    }

    fn get_memory_pool_tx(&self, addr: &str) -> Option<Transaction> {
        todo!()
    }

    fn get_memory_pool(&self) -> HashMap<String, Transaction> {
        todo!()
    }

    fn insert_memory_pool(&self, tx: Transaction) {
        todo!()
    }

    fn clear_memory_pool(&self) {
        todo!()
    }

    fn get_best_height(&self) -> Result<i32> {
        todo!()
    }

    fn get_block_hashes(&self) -> Vec<String> {
        todo!()
    }

    fn get_block(&self, block_hash: &str) -> Result<Block> {
        todo!()
    }

    fn verify_tx(&self, tx: &Transaction) -> Result<bool> {
        todo!()
    }

    fn add_block(&self, block: Block) -> Result<()> {
        todo!()
    }

    fn mine_block(&self, txs: Vec<Transaction>) -> Result<Block> {
        todo!()
    }

    fn utxo_reindex(&self) -> Result<()> {
        todo!()
    }

    fn send_data(&self, addr: &str, data: &[u8]) -> Result<()> {
        todo!()
    }

    fn request_blocks(&self) -> Result<()> {
        todo!()
    }

    fn send_block(&self, addr: &str, block: &Block) -> Result<()> {
        todo!()
    }

    fn send_addr(&self, addr: &str) -> Result<()> {
        todo!()
    }

    fn send_inv(&self, addr: &str, kind: &str, items: Vec<String>) -> Result<()> {
        todo!()
    }

    fn send_get_blocks(&self, addr: &str) -> Result<()> {
        todo!()
    }

    fn send_get_data(&self, addr: &str, kind: &str, id: &str) -> Result<()> {
        todo!()
    }

    pub fn send_tx(&self, addr: &str, tx: &Transaction) -> Result<()> {
        todo!()
    }

    fn send_version(&self, addr: &str) -> Result<()> {
        todo!()
    }

    fn handle_version(&self, msg: VersionMsg) -> Result<()> {
        todo!()
    }

    fn handle_addr(&self, msg: Vec<String>) -> Result<()> {
        todo!()
    }

    fn handle_block(&self, msg: BlockMsg) -> Result<()> {
        todo!()
    }

    fn handle_inv(&self, msg: InvMsg) -> Result<()> {
        todo!()
    }

    fn handle_get_blocks(&self, msg: GetBlockMsg) -> Result<()> {
        todo!()
    }

    fn handle_get_data(&self, msg: GetDataMsg) -> Result<()> {
        todo!()
    }

    fn handle_tx(&self, msg: TxMsg) -> Result<()> {
        todo!()
    }

    fn handle_connection(&self, mut stream: TcpStream) -> Result<()> {
        todo!()
    }
}

fn cmd_to_bytes(cmd: &str) -> [u8; CMD_LEN] {
    todo!()
}

fn bytes_to_cmd(bytes: &[u8]) -> Result<Message> {
    todo!()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_cmd() {}
}
