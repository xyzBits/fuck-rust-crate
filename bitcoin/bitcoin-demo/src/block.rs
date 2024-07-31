use std::ops::Bound::Unbounded;
use std::time::SystemTime;

use crypto::digest::Digest;
use crypto::sha2::Sha256;
use env_logger::fmt::Color::{Black, White};
use log::info;
use serde::{Deserialize, Serialize};

use crate::errors::Result;
use crate::transaction::Transaction;

#[allow(non_snake_case)]
#[allow(unused)]
//! Block implement of blockchain
const TARGET_HEXS: usize = 4;


// Block keeps block headers
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    timestamp: u128,
    transactions: Vec<Transaction>,
    prev_block_hash: String,

    // 区块哈段可以唯一、明确地标识一个区块，并且任何节点哈希计算都可以独立地获取该区块哈希值
    hash: String,

    // A counter used for the proof-of-work algorithm
    nonce: i32,
    height: i32,
    // Why no Merkle Root hash ?
}

impl Block {
    // 使用 self 就不会把的有权转移
    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }

    pub fn get_prev_hash(&self) -> String {
        self.prev_block_hash.clone()
    }

    pub fn get_transaction(&self) -> &Vec<Transaction> {
        &self.transactions
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn new_block(
        transactions: Vec<Transaction>,
        prev_block_hash: String,
        height: i32,
    ) -> Result<Block> {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_millis();

        let mut block =
            Block {
                timestamp,
                transactions,
                prev_block_hash,
                hash: String::new(),
                nonce: 0,
                height,
            };

        block.run_proof_of_work()?; // 不断循环，直到挖到为止

        Ok(block)
    }

    /// NewGenesisBlock creates and returns genesis Block
    pub fn new_genesis_block(coinbase: Transaction) -> Block {

        Block::new_block(
            vec![coinbase],
            String::new(),
            0)
            .unwrap()
    }

    /// Run performs a proof-of-work
    /// 矿工可以不断区块的 nonce 值（随机数），来寻找满足条件的哈希值，这是实现工作量证明算法的关键步骤
    /// 挖矿：不断改变 block.nonce 的值，直到找到一个使 validate 方法返回 true 的 nonce
    /// 只要调用这个函数，就会不断循环，直到按到合适的 nonce 才停止
    ///
    fn run_proof_of_work(&mut self) -> Result<()> {
        info!("Mining the block");
        while !self.validate()? {
            self.nonce += 1; // 不断地改变 Block 的 nonce
        }

        // 运行到这里，说明找到了符合条件的 nonce
        let data = self.prepare_hash_data()?;

        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
        self.hash = hasher.result_str();

        Ok(())
    }

    fn hash_transactions(&self) -> Result<Vec<u8>> {
        todo!()
    }

    fn prepare_hash_data(&self) -> Result<Vec<u8>> {
        todo!()
    }


    fn validate(&self) -> Result<bool> {
        todo!()
    }
}