use std::collections::HashMap;
use data_encoding::HEXLOWER;
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
        UTXOSet {
            blockchain
        }
    }

    pub fn get_blockchain(&self) -> &Blockchain {
        &self.blockchain
    }

    /// 找到未花费的输出
    pub fn find_spendable_outputs(
        &self,
        pub_key_hash: &[u8],
        amount: i32,
    ) -> (i32, HashMap<String, Vec<usize>>) {
        let mut unspent_outputs: HashMap<String, Vec<usize>> = HashMap::new();
        let mut accmulated = 0;

        let db = self.blockchain.get_db();
        let utxo_tree = db.open_tree(UTXO_TREE).unwrap();

        for item in utxo_tree.iter() {
            let (k, v) = item.unwrap();
            let txid_hex = HEXLOWER.encode(k.to_vec().as_slice());
            let outs: Vec<TXOutput> = bincode::deserialize(v.to_vec().as_slice())
                .expect("unable to deserialize TXOutput");

            for (idx, out) in outs.iter().enumerate() {

            }


        }


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
    pub fn reindex(&self) {}


    /// 使用来自区块的交易更新 UTXO 集
    pub fn update(&self, block: &Block) {
        todo!()
    }
}