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
    pub value: i32, // 金额 amount
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
    pub fn new_utxo(
        wallet: &Wallet,
        to: &str,
        amount: i32,
        utxo: &UTXOSet,
    ) -> Result<Transaction> {
        todo!()
    }
}


impl Transaction {
    pub fn hash(&self) -> Result<String> {
        todo!()
    }
}