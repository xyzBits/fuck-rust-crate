use std::collections::HashMap;
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
}