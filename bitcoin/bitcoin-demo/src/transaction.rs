use log::info;
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
    pub fn new_utxo(
        wallet: &Wallet,
        to: &str,
        amount: i32,
        utxo: &UTXOSet,
    ) -> Result<Transaction> {
        info!("new UTXO Transaction from: {}, to: {}", 1, 2);

        // let mut vin = Vec::new();

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


        todo!()
    }
}

impl Transaction {
    pub fn hash(&self) -> Result<String> {
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
}