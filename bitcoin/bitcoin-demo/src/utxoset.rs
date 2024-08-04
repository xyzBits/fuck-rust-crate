use std::collections::HashMap;
use crate::block::Block;
use crate::blockchain::Blockchain;
use crate::errors::Result;
use crate::transaction::TXOutputs;

/// UTXOSet represents UTXO set
/// 设计 UTXO 的意义，使代码的语意更加明确，提高代码的模块化，并可能提供一些特定于 UTXO 集的操作和数据
pub struct UTXOSet {
    blockchain: Blockchain,
}


impl UTXOSet {
    /// FindUnspentTransactions returns a list of transactions containing unspent outputs
    /// 在 UTXO 集合中查找可用于此交易 TXOutputs(struct TXOutputs{outputs: Vec<TXOutput>,}
    /// 返回的是一个 accumulated 和 unspent outputs 的元组
    /// 此方法会遍历所在存储在数据库中的未花费交易输出，找到一个满足以下条件的输出集合
    ///   它们被指定的公钥 pub_key_hash 锁定，且它们的总价值 > amount 参数
    ///   这些输出可以用于创建一个新的交易
    /// 当 accumulated 总价值 > amount 时，即停止查找，直接返回当前找到的 unspent_outputs 进行后续的花费
    pub fn find_spendable_outputs(
        &self,
        pub_key_hash: &[u8],
        amount: i32,
    ) -> Result<(i32, HashMap<String, Vec<i32>>)> {
        let mut unspent_outputs: HashMap<String, Vec<i32>> = HashMap::new();
        let mut accumulated = 0;

        let db = sled::open("data/utxos")?;
        for kv in db.into_iter() { // db 是个可迭代的 obj
            let (k, v) = kv?;
            let txid = String::from_utf8(k.to_vec())?;
            let outs: TXOutputs = bincode::deserialize(&v.to_vec())?;

            for out_idx in 0..outs.outputs.len() {
                // 当 accumulated 总价值 > amount 时，停止查找，直接利用 unspent_outputs 进行后续的花费
                if outs.outputs[out_idx].is_locked_with_key(pub_key_hash)
                    && accumulated < amount {
                    accumulated += outs.outputs[out_idx].value;
                    match unspent_outputs.get_mut(&txid) {
                        Some(v) => v.push(out_idx as i32),
                        None => {
                            unspent_outputs.insert(txid.clone(), vec![out_idx as i32]);
                        }
                    }
                }
            }
        }


        Ok((accumulated, unspent_outputs))
    }

    /// 找到所有被指定公钥哈希锁定的未花费交易输出，这些输出代表了 pub key hash 这个地址的余额
    ///
    pub fn find_utxo(&self, pub_key_hash: &[u8]) -> Result<TXOutputs> {
        let mut utxos = TXOutputs {
            outputs: Vec::new(),
        };

        let db = sled::open("data/utxos")?;

        for kv in db.iter() {
            let (_, v) = kv?;
            let outs: TXOutputs = bincode::deserialize(&v.to_vec())?;

            for out in outs.outputs {
                if out.is_locked_with_key(pub_key_hash) {
                    utxos.outputs.push(out.clone());
                }
            }
        }

        Ok(utxos)
    }

    /// 返回数据库中存储的所有地址的所有未花费交易输出的数量
    pub fn count_transactions(&self) -> Result<i32> {
        let mut counter = 0;

        let db = sled::open("data/utxos")?;

        for kv in db.iter() {
            kv?;
            counter += 1;
        }

        Ok(counter)
    }


    /// 重建 utxo 集合，它首先删除数据库中的所有数据，然后通过查找区块链中的所有未花费交易输出来重新填充数据库
    pub fn reindex(&self) -> Result<()> {
        std::fs::remove_dir("data/utxos").ok();

        let db = sled::open("data/utxos")?;

        let utxos = self.blockchain.find_utxo();

        for (txid, outs) in utxos {
            db.insert(txid.as_bytes(), bincode::serialize(&outs)?)?;
        }

        Ok(())
    }
    
    /// 此方法会用区块中的交易来更新 utxo 集合，对于每个交易，它会从数据库中移除已经被花费的输出
    /// 并添加新的未花费输出
    pub fn update(&self, block: &Block) -> Result<()> {
        let db = sled::open("data/utxos")?;

        for tx in block.get_transaction() {
            if !tx.is_coinbase() {
                for vin in &tx.vin {
                    let mut update_outputs = TXOutputs {
                        outputs: Vec::new(),
                    };

                    let outs: TXOutputs = bincode::deserialize(&db.get(&vin.txid)?.unwrap().to_vec())?;
                    for out_idx in 0..outs.outputs.len() {
                        if out_idx != vin.vout as usize {
                            update_outputs.outputs.push(outs.outputs[out_idx].clone());
                        }
                    }

                    if update_outputs.outputs.is_empty() {
                        db.remove(&vin.txid)?;
                    } else {
                        db.insert(vin.txid.as_bytes(), bincode::serialize(&update_outputs)?)?;
                    }

                }
            }

            let mut new_outputs = TXOutputs {
                outputs: Vec::new(),
            };

            for out in &tx.vout {
                new_outputs.outputs.push(out.clone());
            }

            db.insert(tx.id.as_bytes(), bincode::serialize(&new_outputs)?)?;
        }

        Ok(())
    }
}