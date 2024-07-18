use serde::{Deserialize, Serialize};
use sled::IVec;
use crate::proof_of_work::ProofOfWork;
use crate::transaction::Transaction;
use crate::utils::{current_timestamp, sha256_digest};

#[derive(Clone, Serialize, Deserialize)]
pub struct Block {
    // 当前时间戳，也就是区块创建的时间
    timestamp: i64,

    // 当一个块的哈希，即父哈希
    pre_block_hash: String,

    // 当前块的哈希
    hash: String,

    // 区块存储的实际有效信息，也就是交易
    transactions: Vec<Transaction>,
    nonce: i64,
    height: usize,

}

impl Block {
    /// 新建一个区块
    pub fn new_block(pre_block_hash: String, transactions: &[Transaction], height: usize) -> Self {
        let mut block = Block {
            timestamp: current_timestamp(),
            pre_block_hash,
            hash: String::new(),
            transactions: transactions.to_vec(),
            nonce: 0,
            height,
        };

        // 挖矿计算 哈希
        let pow = ProofOfWork::new_proof_of_work(block.clone());
        let (nonce, hash) = pow.run();

        block.nonce = nonce;
        block.hash = hash;
        block
    }

    /// 从字节数组反序列化
    pub fn deserialize(bytes: &[u8]) -> Block {
        bincode::deserialize(bytes).unwrap()
    }


    /// 区块序列化
    pub fn serialized(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap().to_vec()
    }

    /// 生成创世块
    pub fn generate_genesis_block(transaction: &Transaction) -> Block {
        let transactions = vec![transaction.clone()];
        Block::new_block(String::from("None"), &transactions, 0)
    }

    /// 计算区块里所有交易的哈希
    pub fn hash_transactions(&self) -> Vec<u8> {
        let mut txhashs = vec![];
        for transaction in &self.transactions {
            txhashs.extend(transaction.get_id());
        }

        sha256_digest(txhashs.as_slice())
    }

    pub fn get_transactions(&self) -> &[Transaction] {
        self.transactions.as_slice()
    }

    pub fn get_pre_block_hash(&self) -> String {
        self.pre_block_hash.clone()
    }

    pub fn get_hash(&self) -> &str {
        self.hash.as_ref()
    }

    pub fn get_hash_bytes(&self) -> Vec<u8> {
        self.hash.as_bytes().to_vec()
    }

    pub fn get_timestamp(&self) -> i64 {
        self.timestamp
    }

    pub fn get_height(&self) -> usize {
        self.height
    }
}

impl From<Block> for IVec {
    fn from(block: Block) -> Self {
        let bytes = bincode::serialize(&block).unwrap();
        Self::from(bytes)
    }
}


#[cfg(test)]
mod tests {
    use crate::transaction::Transaction;
    use super::Block;

    #[test]
    fn test_new_block() {
        let block = Block::new_block(
            String::from("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"),
            &vec![],
            0,
        );

        let vec1 = block.serialized();
        println!("new block hash is {}", block.hash)
    }

    #[test]
    fn test_block_serialize() {
        let tx = Transaction::new_coinbase_tx("Genesis");

        let block = Block::new_block(
            String::from("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"),
            &vec![tx],
            0,
        );
        let block_bytes = block.serialized();
        let desc_block = Block::deserialize(&block_bytes[..]);
        assert_eq!(block.hash, desc_block.hash)
    }
}