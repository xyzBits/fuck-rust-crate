use std::collections::HashMap;
use std::slice::RSplit;

use failure::format_err;
use log::{error, info};
use rand::rngs::OsRng;
use rand::RngCore;
use serde::{Deserialize, Serialize};

use crate::errors::*;
use crate::utxoset::UTXOSet;
use crate::wallets::*;

const SUBSIDY: i32 = 10;

/// TXInput represents a transaction input
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TXInput {
    pub txid: String,
    // index 序号，-1 表示 coinbase tx ，cbtx
    pub vout: i32,
    // 签名，其他人可以通过 Alice 的 pub key 和整个消息体，来验证 alice 的 signature 是 否有效
    pub signature: Vec<u8>,
    pub pub_key: Vec<u8>, // 消费者 Alice 的公钥
}

/// TXOutput represents a transaction output
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TXOutput {
    pub value: i32,            // 金额 amount
    pub pub_key_hash: Vec<u8>, // 标识了谁能花费这笔 TXOutput
}

/// TXOutputs collects TXOutput
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TXOutputs {
    pub outputs: Vec<TXOutput>, // UTXO 集合
}

/// Transaction represents a Bitcoin transaction
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub id: String,
    pub vin: Vec<TXInput>,
    pub vout: Vec<TXOutput>,
}

impl Transaction {
    /// New utxo transaction creates a new transaction
    /// 它从一个给定的钱包创建一个新的 utxo 未消费的交易输出，发送指定数量的资金到指定的地址，并返回创建的交易
    pub fn new_utxo(wallet: &Wallet, to: &str, amount: i32, utxo: &UTXOSet) -> Result<Transaction> {
        info!("new UTXO Transaction from: {}, to: {}", 1, 2);

        let mut vin = Vec::new();

        // 复制钱包的公钥后，创建公钥的哈希
        let mut pub_key_hash = wallet.public_key.clone();
        hash_pub_key(&mut pub_key_hash);

        // 在该公钥地址的 utxo 集合中迭代累计查找可用于此交易的 TXOutput
        // 返回一个包含总额和刚刚好覆盖该笔交易的映射的元组
        // 例子： 某笔交易需要 10 BTC
        // 遍历 msg.sender 的 UTXO outputs
        // 第一笔 3 BTC， 第二笔 5 BTC， 第三笔 4 BTC
        // 此时 3 + 5 + 4 = 12， 刚刚好也是第一次 < 10
        // 这三笔 TXOutput 就会用来全部花费掉以执行本次交易：
        // 转给 receiver 10 BTC，转给 msg.sender 2 BTC
        let acc_v = utxo.find_spendable_outputs(&pub_key_hash, amount)?;

        // 如果交易金额  > 找到的所有总额，则记录 Not Enough balance 错误并返回错误
        if acc_v.0 < amount {
            error!("Not Enough balance");
            return Err(format_err!(
                "Not Enough balance: current balance {}",
                acc_v.0
            ));
        }

        // 对于每个待被花费的 TXOutput 和其对应的输出索引，创建一个新的 TXInput 交易输入
        // 并将其添加到交易输入列表中
        for tx in acc_v.1 {
            for out in tx.1 {
                // 迭代 Vec<TXOutput>
                let input = TXInput {
                    txid: tx.0.clone(),
                    vout: out,
                    signature: Vec::new(),
                    pub_key: wallet.public_key.clone(),
                };

                vin.push(input);
            }
        }

        // 创建交易输出
        // 首先，创建一个新的交易输出，其金额等于交易金额，收款人是目标地址
        // 如果找到的总额大于交易金额（意味着要找零），则创建一个新的交易输出，
        // 其金额等于找到的总额减去交易金额
        // 收款人是发送者的地址
        let mut tx_outs = vec![TXOutput::new(amount, to.to_string())?];
        if acc_v.0 > amount {
            tx_outs.push(TXOutput::new(acc_v.0 - amount, wallet.get_address())?);
        }

        // 创建新的交易，其中包括已创建的交易输入和输出，然后计算并设置交易 id
        let mut tx = Transaction {
            id: String::new(),
            vin,
            vout: tx_outs,
        };

        tx.id = tx.hash()?;
        // 最后对交易进行签名
        utxo.blockchain
            .sign_transaction(&mut tx, &wallet.secret_key)?;

        Ok(tx)
    }

    /// Coinbase TX 不需要通过挖矿来创建，实际上，coinbase 交易就是矿工因成功挖掘出新的区块而得到的奖励，
    ///  这个奖励直接发给矿工，而不是从其他交易中取得
    pub fn new_coinbase(to: String, mut data: String) -> Result<Transaction> {
        info!("new coinbase Transaction to :{}", to);

        let mut key = [0u8; 32];
        if data.is_empty() {
            let mut rand = OsRng::default();
            rand.fill_bytes(&mut key);
            data = format!("Reward to '{}'", to); // 挖矿奖励
        }

        let mut pub_key = Vec::from(data.as_bytes());
        pub_key.append(&mut Vec::from(key));

        let mut tx = Transaction {
            id: String::new(),
            vin: vec![TXInput {
                txid: String::new(),
                vout: -1,
                signature: Vec::new(),
                pub_key,
            }],
            vout: vec![TXOutput::new(SUBSIDY, to)?], // 10 BTC 作为 mine reward
        };
        tx.id = tx.hash()?;

        Ok(tx)
    }

    pub fn is_coinbase(&self) -> bool {
        self.vin.len() == 1 && self.vin[0].txid.is_empty() && self.vin[0].vout == -1
    }

    /// 验证交易输入的签名，它将当前交易中的每个输入和先前交易相关联，并确认 输入的签名与先前交易的输出一致
    pub fn verify(&self, prev_txs: HashMap<String, Transaction>) -> Result<bool> {
        if self.is_coinbase() {
            return Ok(true); // 它没有先前的输入，因此可以立即确认签名是有效的
        }

        for vin in &self.vin {
            if prev_txs.get(&vin.txid).unwrap().id.is_empty() {
                return Err(format_err!("ERROR: Previous transaction is not correct"));
            }
        }

        // 创建当前交易的一个修剪版副本，它移除了输入签名和公钥，以便我们可以重新计算它的哈希值
        let mut tx_copy = self.trim_copy();

        for in_id in 0..self.vin.len() {}

        Ok(true)
    }

    pub fn sign(
        &mut self,
        private_key: &[u8],
        prev_txs: HashMap<String, Transaction>,
    ) -> Result<()> {
        todo!()
    }

    pub fn hash(&self) -> Result<String> {
        todo!()
    }

    fn trim_copy(&self) -> Transaction {
        todo!()
    }
}

impl TXOutput {
    /// 定义 TXOutput 交易输出的方法
    /// 在区块链交易中， 锁定交易输出实际上就是将交易输出与一个特定的公钥哈希（也就是地址）相关联
    /// 这个步骤只是确保了只拥有相应私钥的用户才能在将来的交易中使用这个输出
    ///
    /// 检查当前输出是否被给定的公钥哈希锁定，如果当前输出的公钥哈希与给定的公钥哈希相等，
    /// 那么返回 true 否则返回 false
    pub fn is_locked_with_key(&self, pub_key_hash: &[u8]) -> bool {
        self.pub_key_hash == pub_key_hash
    }

    fn lock(&mut self, address: &str) -> Result<()> {
        todo!()
    }

    pub fn new(value: i32, address: String) -> Result<Self> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_signature() {}
}
