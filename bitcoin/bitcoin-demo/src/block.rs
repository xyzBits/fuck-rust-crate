use std::ops::Bound::Unbounded;
use std::time::SystemTime;

use crypto::digest::Digest;
use crypto::sha2::Sha256;
use env_logger::fmt::Color::{Black, White};
use log::info;
use merkle_cbt::CBMT;
use merkle_cbt::merkle_tree::Merge;
use serde::{Deserialize, Serialize};

use crate::errors::Result;
use crate::transaction::Transaction;

#[allow(non_snake_case)]
#[allow(unused)]
// Block implement of blockchain
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

    /// HashTransactions returns a hash of the transactions in the block
    /// 将区块中的所有交易哈希化，然后用 Merkle 树组合这些哈希，得到一个树根，这个树根是对区块中所有交易的一个摘要
    fn hash_transactions(&self) -> Result<Vec<u8>> {
        let mut transactions = Vec::new();
        for tx in &self.transactions {
            transactions.push(tx.hash()?.as_bytes().to_owned());
        }

        let tree = CBMT::<Vec<u8>, MergeVu8>::build_merkle_tree(&transactions);

        Ok(tree.root())
    }

    /// 将 {上一个区块 hash. 挖到符合条件的 nonce ，时间戳， 交易数据} 等结构化数据序列化成字节流 Vec<u8> 返回
    /// 这一步是为了将目前的 Block 的数据整体做 hash () ，作为未来下一个区块中的 previous block hash 字段的内容
    fn prepare_hash_data(&self) -> Result<Vec<u8>> {
        let content = (
            self.prev_block_hash.clone(),
            self.hash_transactions()?,
            self.timestamp,
            TARGET_HEXS,
            self.nonce
        );

        let bytes = bincode::serialize(&content)?;
        Ok(bytes)
    }


    /// Validate validates block's POW
    /// 本 Block 的 nonce 不断变化，不断调用该函验证该 nonce 是否能够让 哈希值满足某种条件
    /// hash 完成后 target hexs 位为 0
    fn validate(&self) -> Result<bool> {
        let data = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
        let mut vec1 = Vec::new();
        // [48, 48, 48, 48] 48 是 '0' utf-8 的编码结果，
        // hash 完后 target-hexs 位为0
        vec1.resize(TARGET_HEXS, '0' as u8);

        // 比较哈希值的前 target hexs 位和合零向量是否相待，如果相等，证明 nonce 有效
        Ok(&hasher.result_str()[0..TARGET_HEXS] == String::from_utf8(vec1)?)
    }
}

/// 只是一个实现特定功能的载体
struct MergeVu8 {}

/// 接收两个 Vec<u8> 类型的引用，将它们合并后，用 SHA256 生成 hash value
impl Merge for MergeVu8 {
    type Item = Vec<u8>; // 在本实现中，我们处理的数据类型是 Vec<u8>

    fn merge(left: &Self::Item, right: &Self::Item) -> Self::Item {
        let mut re = [0u8; 32];

        let mut hasher = Sha256::new();
        let mut data = left.clone();
        data.append(&mut right.clone());

        hasher.input(&data);
        hasher.result(&mut re); // 将 hash 结果吐到 re 中

        re.to_vec() //
    }
}

#[cfg(test)]
mod tests {
    use crypto::digest::Digest;
    use crypto::sha2::Sha256;

    use crate::block::TARGET_HEXS;

    #[test]
    fn test_serialize() {
        let message = "hello wold".to_string();
        let data = [0u8; 8];
        let number = 45;

        let content = (message, data, number);
        let bytes = bincode::serialize(&content).unwrap();

        // let mut input = vec![];
        // input
        //     .extend(message.as_bytes())
        //     .extend(&data)
        //     .extend(number.to_be_bytes());
        //
        // let output = bincode::serialize(input).unwrap();
        //
        // assert_eq!(bytes, output);
    }

    #[test]
    fn test_validate() {
        let mut data = Vec::new();
        data.resize(TARGET_HEXS, '0' as u8);
        println!("{}", '0' as u8);
        println!("{:?}", data);
    }


    #[test]
    fn test_sha256() {
        let message = "hello world";
        let mut hasher = Sha256::new();
        hasher.input(message.as_bytes());
        let hash = hasher.result_str();
        println!("{}", hash);
    }
}