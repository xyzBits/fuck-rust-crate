use std::collections::HashMap;
use std::num::ParseFloatError;
use std::sync::{Arc, RwLock};
use sled::{Db, Tree};
use crate::block::Block;
use crate::transaction::{Transaction, TXOutput};

const TIP_BLOCK_HASH_KEY: &str = "tip_block_hash";
const BLOCK_TREE: &str = "blocks";


#[derive(Clone)]
pub struct Blockchain {
    tip_hash: Arc<RwLock<String>>, // hash of last block
    db: Db,

}


impl Blockchain {

    /// 创建新的区块链
    pub fn create_blockchain(genesis_address: &str) -> Blockchain {
        todo!()
    }

    pub fn update_blocks_tree(blocks_tree: &Tree, block: &Block) {
        todo!()
    }

    /// 创建区块链实例
    pub fn new_blockchain() -> Blockchain {
        todo!()
    }

    pub fn get_db(&self) -> &Db {
        &self.db
    }

    pub fn get_tip_hash(&self) -> String {
        todo!()
    }

    pub fn set_tip_hash(&self, new_tip_hash: &str) {
        todo!()
    }


    /// 挖矿新区块
    pub fn mine_block(&self, transaction: &[Transaction]) -> Block {
        todo!()
    }


    pub fn iterator(&self) -> BlockchainIterator {
        todo!()
    }

    /// 查找所有未花费的交易输出  (k -> txid_hex, v -> Vec<TXOutput>)
    pub fn find_utxo(&self) -> HashMap<String, Vec<TXOutput>> {
        todo!()
    }


    /// 从区块链中查找交易
    pub fn find_transaction(&self, txid: &[u8]) -> Option<Transaction> {
        todo!()
    }

    pub fn add_block(&self, block: &Block) {
        todo!()
    }


    /// 获取最新区块在链中的高度
    pub fn get_best_height(&self) -> usize {
        todo!()
    }


    // 返回链中所有区块的哈希列表
    pub fn get_block_hashed(&self) -> Vec<Vec<u8>> {
        todo!()
    }

}


pub struct BlockchainIterator {
    db: Db,
    current_hash: String,
}


impl BlockchainIterator {
    pub fn new(tip_hash: String, db: Db) -> BlockchainIterator {
        todo!()
    }

    pub fn next(&mut self) -> Option<Block> {
        todo!()
    }
}













