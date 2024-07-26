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
        let mut accumulated = 0;

        let db = self.blockchain.get_db();
        let utxo_tree = db.open_tree(UTXO_TREE).unwrap();

        for item in utxo_tree.iter() {
            let (k, v) = item.unwrap();
            let txid_hex = HEXLOWER.encode(k.to_vec().as_slice());
            let outs: Vec<TXOutput> = bincode::deserialize(v.to_vec().as_slice())
                .expect("unable to deserialize TXOutput");

            for (idx, out) in outs.iter().enumerate() {
                if out.is_locked_with_key(pub_key_hash) && accumulated < amount {
                    accumulated += out.get_value();
                    if unspent_outputs.contains_key(txid_hex.as_str()) {
                        unspent_outputs
                            .get_mut(txid_hex.as_str())
                            .unwrap()
                            .push(idx);
                    } else {
                        unspent_outputs.insert(txid_hex.clone(), vec![idx]);
                    }
                }
            }
        }


        (accumulated, unspent_outputs)
    }

    /// 通过公钥哈希查找 UTXO 集合
    pub fn find_utxo(&self, pub_key_hash: &[u8]) -> Vec<TXOutput> {
        let db = self.blockchain.get_db();
        let utxo_tree = db.open_tree(UTXO_TREE).unwrap();
        let mut utxos = vec![];
        for item in utxo_tree.iter() {
            let (_, v) = item.unwrap();
            let outs: Vec<TXOutput> = bincode::deserialize(v.to_vec().as_slice())
                .expect("Unable to deserialize TXOutput");

            for out in outs.iter() {
                if out.is_locked_with_key(pub_key_hash) {
                    utxos.push(out.clone());
                }
            }
        }

        utxos
    }

    /// 统计 UTXO 集合中的交易数量
    pub fn count_transactions(&self) -> i32 {
        let db = self.blockchain.get_db();
        let utxo_tree = db.open_tree(UTXO_TREE).unwrap();
        let mut counter = 0;
        for _ in utxo_tree.iter() {
            counter += 1;
        }

        counter
    }


    /// 重建 UTXO 集合
    pub fn reindex(&self) {
        let db = self.blockchain.get_db();
        let utxo_tree = db.open_tree(UTXO_TREE).unwrap();
        let _ = utxo_tree.clear().unwrap();

        let utxo_map = self.blockchain.find_utxo();
        for (txid_hex, outs) in &utxo_map {
            let txid = HEXLOWER.decode(txid_hex.as_bytes()).unwrap();
            let value = bincode::serialize(outs).unwrap();

            let _ = utxo_tree.insert(txid.as_slice(), value).unwrap();
        }
    }


    /// 使用来自区块的交易更新 UTXO 集
    pub fn update(&self, block: &Block) {
        todo!()
    }
}