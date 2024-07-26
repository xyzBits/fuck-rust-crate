use std::collections::HashMap;
use std::num::ParseFloatError;
use serde::{Deserialize, Serialize};

/// UTXO 未花费交易输出
/// 与传统的银行账户系统采用的账户余额模型不同
/// UTXO 模型为比特币网络提供了更高的安全性和隐私性
///
/// UTXO 模型简介，
/// 在UTXO中，用户的余额不是存储在单一的账户中，而是分散在多个UTXO中，
/// 每个UTXO 代表比特币网络上一笔可用于未来交易的输出 ，
/// 当用户发起一笔新交易时，他们实际上是将之前交易中的一个或者多个UTXO作为输入
/// 转化为新的UTXO 给接收方，
/// 这种模型的一个关键特点是，UTXO一旦被消费，便不会再次出现 在交易输入中
///
/// UTXO 模型的优势
/// UTXO 提供的不仅是一种记录资金所有权和转移的方法，它还增强了网络的隐私性和安全性，
/// 由于UTXO 可以被分割和合并，但不可更改，
/// 这使得追踪资金流动变得更加复杂，从而提高了用户隐私，
/// 此外，由于UTXO 模型允许网络并行验证交易，大大提高了比特币网络的处理能力的效率
///
/// 交易处理流程
/// 一笔比特币交易从生成到确认过程涉及到多个步骤，
/// 首先，交易发起者需要选择足够的UTXO 作为输入来覆盖交易的总额和手续费
/// 然后，他们生成新的TxOut作为输出 ，指定接收方和金额，
/// 之后，发起者使用他们的私钥对交易进行签名，
/// 最后，这笔交易被广播到比特币网络，经过网络节点的验证后，
/// 被包含在一个区块中，最终 被添加到区块链上
///
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Transaction {
    // 交易的版本号，用于指示交易的格式或规则，允许比特币网络升级交易格式而保持向后兼容
    version: i32,

    // 一个 TxIn 结构体的数组，代表交易的输入，每个输入是对之前交易的引用，即使用之前交易的输出作为当前交易的输入
    inputs: Vec<TxIn>,

    // 一个TxOut 结构体的向量，代表交易的输出，每个输出定义了新的资金如何被分配和锁定
    outputs: Vec<TxOut>,

    //交易锁定的时候，定义了交易最早可以被添加到区块链中的时间，如果为0，则表示交易可以立即被包含
    // 否则 ，它可以是一个特定的区块高度或时间戳，交易只能在该时间或之后被处理
    lock_time: u32,
}


/// 交易输入结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TxIn {
    // outPoint 结构体实例，指向一个之前交易的特定输出，即这个输入所引用的UTXO
    previous_output: OutPoint,

    // 解锁脚本，用于证明交易发起者有权使用引用的UTXO，这是验证交易合法性的关键部分
    script_sig: Vec<u8>, // 解锁脚本

    // 序列号，提供了交易的额外灵活性，如替换能力和相对时间锁定
    sequence: u32, // 序列号
}

/// 交易输出结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TxOut {
    // 输出的价值，以聪为单位，聪是比特币的最小单位，1BTC = 10^9 聪
    value: u64, // 输出的价值，单位是聪

    // 锁定脚本，指定了谁可以使用这笔输出 ，这通常包含了接收方的地址信息，确保只有特定的个人
    // 或者条件解锁并花费这些资金
    script_pubkey: Vec<u8>, // 锁定脚本
}

/// 指向特定交易输出的结构体
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
struct OutPoint {
    // 引用交易的 ID，是一个特定交易的唯一标识符，这通常是该交易内容的哈希值
    txid: Vec<u8>, // 引用的交易ID

    // 引用的输出索引 ，指出 txid所指交易的哪一个输出 被当前交易的输入所引用
    vout: u32, // 引用的输出索引
}

/// 代表UTXO集合的结构体
struct UTXOSet {
    // 一个由 OutPoint 到 TxOut 映射，存储了网络上所有未被花费的输出
    utxos: HashMap<OutPoint, TxOut>,
}


/// 结构体实现方法

impl Transaction {
    fn new(version: i32,
           inputs: Vec<TxIn>,
           outputs: Vec<TxOut>,
           lock_time: u32) -> Self {
        Self {
            version,
            inputs,
            outputs,
            lock_time,
        }
    }
    // 更复杂的方法，如签名验证，将需要额外的实现细节
}

impl TxIn {
    pub fn new(previous_output: OutPoint,
               script_sig: Vec<u8>,
               sequence: u32) -> Self {
        Self {
            previous_output,
            script_sig,
            sequence,
        }
    }
}

impl TxOut {
    fn new(value: u64, script_pubkey: Vec<u8>) -> Self {
        Self {
            value,
            script_pubkey,
        }
    }
}















