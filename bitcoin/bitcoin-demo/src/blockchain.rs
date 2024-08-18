use crate::block::Block;
use crate::errors::Result;
use crate::transaction::{TXOutputs, Transaction};
use std::collections::HashMap;

pub struct Blockchain {
    // 字段前的 pub。即使 blockchain 已经声明 pub，如果字段 tip db 不声明，也仍是默认私有的
    pub tip: String,  // 存放 last hash 即最后一个区块的哈希
    pub db: sled::Db, // 区块链 db 中每一个区块都是通过其哈希来索引的，同时 db 上还有一个特殊的 key last 用来存储最后一个区块的哈希
}

pub struct BlockchainIterator<'a> {
    current_hash: String,
    // bc 是指向 blockchain 的引用，lifetime 标告诉编译器：引用的有效期至少和 'a 一样长
    bc: &'a Blockchain,
}

impl Blockchain {
    pub fn new() -> Result<Blockchain> {
        todo!()
    }

    /// 化身中本聪，创建比特币区块链，第一步，创建创始区块
    /// 先清空本地数据库，创建一个新的 db 文件，并添加一个创世区块，该区块包含一个 coinbase 交易
    /// cbtx
    pub fn create_blockchain(address: String) -> Result<Blockchain> {
        todo!()
    }

    /// 挖掘一个新的区块，调用 new block run proof of work 不断循环，直到挖到合适的 nonce
    pub fn mine_block(&mut self, transactions: Vec<Transaction>) -> Result<Block> {
        todo!()
    }

    /// 为 blockchain 定义 iter() 方法，辅助后面 iterator 的实现
    /// blockchain 没实现 iterator trait 但它通过实现这个 iter 方法 returns blockchainIterator 迭代器
    ///
    pub fn iter(&self) -> BlockchainIterator {
        todo!()
    }

    pub fn find_utxo(&self) -> HashMap<String, TXOutputs> {
        todo!()
    }

    pub fn find_transaction(&self, id: &str) -> Result<Transaction> {
        todo!()
    }

    fn get_prev_txs(&self, tx: &Transaction) -> Result<HashMap<String, Transaction>> {
        todo!()
    }

    pub fn sign_transaction(&self, tx: &mut Transaction, private_key: &[u8]) -> Result<()> {
        todo!()
    }

    pub fn verify_transaction(&self, tx: &Transaction) -> Result<bool> {
        todo!()
    }

    pub fn add_block(&mut self, block: Block) -> Result<()> {
        todo!()
    }

    pub fn get_block(&self, block_hash: &str) -> Result<Block> {
        todo!()
    }

    pub fn get_best_height(&self) -> Result<i32> {
        todo!()
    }

    pub fn get_block_hash(&self) -> Vec<String> {
        todo!()
    }
}

/// 为 BlockchainIterator 实现 iterator trait
///
impl<'a> Iterator for BlockchainIterator<'a> {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add_block() {}
}
