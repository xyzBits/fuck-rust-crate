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
    /// 持久化，new_blockchain 会创建一个新的 Blockchain 实例，并向其中加入创世块，
    ///     1. 打开一个数据库文件
    ///     2. 检查文件里面是否已经存储了一个区块链
    ///     3. 如果已经存储了一个区块链
    ///         3-1. 创建一个新的 Blockchain 实例
    ///         3-2. 设置Blockchain 实例的ti为数据库中存储的最后一个块的哈希
    ///     4. 如果没有区块链
    ///         4-1. 创建创世区块
    ///         4-2. 存储到数据库
    ///         4-3. 将创世区块哈希保存为最后一个块的哈希
    ///         4-4. 创建一个新的 Blockchain 实例，初始时tip指向创世块，tip 有尾部，尖端的意思 ，在这里，tip存储的是最后一个块的哈希
    ///
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

/// 产生的所有块都会被保存到一个数据库里面，所以我们可以重新打开一个链，
/// 然后向里面加入新块，但是在实现这一点后，
/// 我们失去了之前非常好的特性，再也无法打印区块链的区块了，
/// 因为现在不是将区块存储在一个数组，而是放到了数据库里面
///
/// DB 允许对一个 bucket 里所有的 key 进行迭代，但是所有的key 以字节序进行存储，
/// 而且我们想要以区块能够进入区块链中的顺序进行打印，此外，我们不想所有的的区块都加载到内存中，
/// 因为区块链的数据库可能很大，
/// 我们将会一个一个地读取它们，因此，需要一个区块链迭代器
///
pub struct BlockchainIterator {
    db: Db,
    current_hash: String,
}

/// 每当需要对链中的块进行迭代时，就会创建一个迭代器，里面存储了当前迭代的块哈希和数据库的连接
/// 通过数据库连接，迭代器逻辑上被附属到一个区块链上，
/// 这晨的区块链是存储了一个数据库连接的 blockchain 实例，
/// 并且通过 blockchain 方法进行创建
/// 注意，迭代器的初始状态为链中的tip，因此 区块将从尾到头，创世块称为头，也就是从最新的到最旧的进行获取，
/// 实际上，选择一个tip就是意味着给一条链投票，一个链可能有多个分支，最长的那条链就会被认为是主分支，
/// 在获得一个tip之后中，可以重新构造整条链，
impl BlockchainIterator {
    pub fn new(tip_hash: String, db: Db) -> BlockchainIterator {
        todo!()
    }

    pub fn next(&mut self) -> Option<Block> {
        todo!()
    }
}













