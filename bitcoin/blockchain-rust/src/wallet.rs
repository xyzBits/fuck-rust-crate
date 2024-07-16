use serde::{Deserialize, Serialize};

const VERSION: u8 = 0x00;
pub const ADDRESS_CHECK_SUM_LEN: usize = 4;

#[derive(Clone, Serialize, Deserialize)]
pub struct Wallet {
    pbkcss8: Vec<u8>,

    // 原生的公钥
    public_key: Vec<u8>,
}


impl Wallet {

    /// 创建一个钱包
    pub fn new() -> Self {
        todo!()
    }
}


























