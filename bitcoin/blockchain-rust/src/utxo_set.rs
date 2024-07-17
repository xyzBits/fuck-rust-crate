use std::collections::HashMap;
use crate::block::Block;
use crate::blockchain::Blockchain;
use crate::transaction::TXOutput;

const UTXO_TREE: &str = "chainstate";

/// UTXO 集合
pub struct UTXOSet {
    blockchain: Blockchain,
}

impl UTXOSet {

    /// 创建 UTXO 集合
    pub fn new(blockchain: Blockchain) -> UTXOSet {
        todo!()
    }

    pub fn get_blockchain(&self) -> &Blockchain {
        todo!()
    }

    /// 找到未花费的输出
    pub fn find_spendable_outputs(
        &self,
        pub_key_hash: &[u8],
        amount: i32
    ) -> (i32, HashMap<String, Vec<usize>>) {
        todo!()
    }

    /// 通过公钥哈希查找 UTXO 集合
    pub fn find_utxo(&self, pub_key_hash: &[u8]) -> Vec<TXOutput> {
        todo!()
    }

    /// 统计 UTXO 集合中的交易数量
    pub fn count_transactions(&self) -> i32 {
        todo!()
    }


    /// 重建 UTXO 集合
    pub fn reindex(&self) {

    }


    /// 使用来自区块的交易更新 UTXO 集
    pub fn update(&self, block: &Block) {
        todo!()
    }




}