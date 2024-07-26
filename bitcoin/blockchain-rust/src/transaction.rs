use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::blockchain::Blockchain;
use crate::utils::base58_decode;
use crate::utxo_set::UTXOSet;
use crate::wallet;

/// 挖矿奖励金
const SUBSIDY: i32 = 10;


/// 交易输入
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TXInput {
    // 一个交易输入引用了前一笔交易的一个输出 ，ID 表明是之前的哪一笔交易
    txid: Vec<u8>,

    // 输出的索引
    vout: usize,

    // 签名
    signature: Vec<u8>,

    // 原生公钥
    pub_key: Vec<u8>,

}

impl TXInput {
    /// 创建一个输入
    pub fn new(txid: &[u8], vout: usize) -> Self {
        Self {
            txid: txid.to_vec(),
            vout,
            signature: vec![],
            pub_key: vec![],
        }
    }

    pub fn get_txid(&self) -> &[u8] {
        self.txid.as_slice()
    }

    pub fn get_vout(&self) -> usize {
        self.vout
    }

    pub fn get_pub_key(&self) -> &[u8] {
        self.pub_key.as_slice()
    }


    /// 检查输入使用了指定密钥来解锁一个输出
    pub fn uses_key(&self, pub_key_hash: &[u8]) -> bool {
        let locking_hash = wallet::hash_pub_key(self.pub_key.as_slice());
        locking_hash.eq(pub_key_hash)
    }
}

/// 交易输出
#[derive(Clone, Serialize, Deserialize)]
pub struct TXOutput {
    // 币的数量
    value: i32,

    // 公钥哈希
    pub_key_hash: Vec<u8>,
}

impl TXOutput {
    /// 创建一个输出
    pub fn new(value: i32, address: &str) -> Self {
        let mut output = Self {
            value,
            pub_key_hash: vec![]
        };

        output.lock(address);
        output
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }

    pub fn get_pub_key_hash(&self) -> &[u8] {
        self.pub_key_hash.as_slice()
    }

    fn lock(&mut self, address: &str)  {
        let payload = base58_decode(address);
        let pub_key_hash = payload[1..payload.len() - wallet::ADDRESS_CHECK_SUM_LEN].to_vec();
        self.pub_key_hash = pub_key_hash;
    }

    pub fn is_locked_with_key(&self, pub_key_hash: &[u8]) -> bool {
        self.pub_key_hash.eq(pub_key_hash)
    }
}

/// 交易
#[derive(Clone, Serialize, Deserialize)]
pub struct Transaction {
    // 交易ID
    id: Vec<u8>,

    // 输入
    vin: Vec<TXInput>,

    // 输出
    vout: Vec<TXOutput>,
}

impl Transaction {

    /// 创建一个 coinbase 交易，该交易没有输入，只有一个输出
    pub fn new_coinbase_tx(to: &str) -> Transaction {
        let tx_output = TXOutput::new(SUBSIDY, to);

        let mut tx_input = TXInput::default();

        tx_input.signature = Uuid::new_v4().as_bytes().to_vec();

        let mut tx = Transaction {
            id: vec![],
            vin: vec![tx_input],
            vout: vec![tx_output]
        };

        tx

    }

    /// 创建一笔 UTXO 交易
    pub fn new_utxo_transaction(
        from: &str,
        to: &str,
        amount: i32,
        utxo_set: &UTXOSet) -> Transaction {
        todo!()
    }

    /// 创建一个修剪后的交易副本
    pub fn trimmed_copy(&self) -> Transaction {
        todo!()
    }

    /// 对交易的每个输入进行签名
    fn sign(&mut self, blockchain: &Blockchain, pkcs8: &[u8]) {
        todo!()
    }


    /// 对交易的每个输入进行签名验证
    pub fn verify(&self, blockchain: &Blockchain) -> bool {
        todo!()
    }


    /// 判断是否为 coinbase 交易
    pub fn is_coinbase(&self) -> bool {
        todo!()
    }

    /// 生成交易的哈希
    fn hash(&mut self) -> Vec<u8> {
        todo!()
    }





























    pub fn get_id(&self) -> &[u8] {
        todo!()
    }

    pub fn get_id_bytes(&self) -> Vec<u8> {
        todo!()
    }

    pub fn get_vin(&self) -> &[TXInput] {
        todo!()
    }


    pub fn get_vout(&self) -> &[TXOutput] {
        todo!()
    }

    pub fn serialize(&self) -> Vec<u8> {
        todo!()
    }

    pub fn deserialized(bytes: &[u8]) -> Transaction {
        todo!()
    }




}