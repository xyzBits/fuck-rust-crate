use std::collections::HashMap;
use std::env::current_dir;
use std::sync::{Arc, RwLock};

use sled::{Db, Tree};
use sled::transaction::TransactionResult;
use crate::block::Block;
use crate::transaction::{Transaction, TXOutput};

const TIP_BLOCK_HASH_KEY: &str = "tip_block_hash";
const BLOCKS_TREE: &str = "blocks";


#[derive(Clone)]
pub struct Blockchain {
    tip_hash: Arc<RwLock<String>>, // hash of last block
    db: Db,

}


impl Blockchain {
    /// 创建新的区块链
    pub fn create_blockchain(genesis_address: &str) -> Blockchain {
        let db = sled::open(current_dir().unwrap().join("data")).unwrap();

        let blocks_tree = db.open_tree(BLOCKS_TREE).unwrap();

        let data = blocks_tree.get(TIP_BLOCK_HASH_KEY).unwrap();
        let tip_hash;
        if data.is_none() {
            let coinbase_tx = Transaction::new_coinbase_tx(genesis_address);
            let block = Block::generate_genesis_block(&coinbase_tx);

            Self::update_blocks_tree(&blocks_tree, &block);
            tip_hash = String::from(block.get_hash());
        } else {
            tip_hash = String::from_utf8(data.unwrap().to_vec()).unwrap();
        }

        Blockchain {
            tip_hash: Arc::new(RwLock::new(tip_hash)),
            db,
        }
    }

    pub fn update_blocks_tree(blocks_tree: &Tree, block: &Block) {
        let block_hash = block.get_hash();
        let _: TransactionResult<(), ()> = blocks_tree.transaction(|tx_db| {
            let _ = tx_db.insert(block_hash, block.clone());
            let _ = tx_db.insert(TIP_BLOCK_HASH_KEY, block_hash);
            Ok(())
        });
    }

    /// 创建区块链实例
    pub fn new_blockchain() -> Blockchain {
        let db = sled::open(current_dir().unwrap().join("data")).unwrap();
        let blocks_tree = db.open_tree(BLOCKS_TREE).unwrap();

        let tip_bytes = blocks_tree
            .get(TIP_BLOCK_HASH_KEY)
            .unwrap()
            .expect("No existing blockchain found. Create one first.");
        let tip_hash = String::from_utf8(tip_bytes.to_vec()).unwrap();

        Blockchain {
            tip_hash: Arc::new(RwLock::new(tip_hash)),
            db,
        }
    }

    pub fn get_db(&self) -> &Db {
        &self.db
    }

    pub fn get_tip_hash(&self) -> String {
        self.tip_hash.read().unwrap().clone()
    }

    pub fn set_tip_hash(&self, new_tip_hash: &str) {
        let mut tip_hash = self.tip_hash.write().unwrap();
        *tip_hash = String::from(new_tip_hash);
    }


    /// 挖矿新区块
    pub fn mine_block(&self, transactions: &[Transaction]) -> Block {
        for transaction in transactions {
            if transaction.verify(self) == false {
                panic!("ERROR: Invalid transaction")
            }
        }

        let best_height = self.get_best_height();
        let block = Block::new_block(self.get_tip_hash(), transactions, best_height + 1);
        let block_hash = block.get_hash();

        let blocks_tree = self.db.open_tree(BLOCKS_TREE).unwrap();
        Self::update_blocks_tree(&blocks_tree, &block);
        self.set_tip_hash(block_hash);

        block
    }


    pub fn iterator(&self) -> BlockchainIterator {
        BlockchainIterator::new(self.get_tip_hash(), self.db.clone())
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













