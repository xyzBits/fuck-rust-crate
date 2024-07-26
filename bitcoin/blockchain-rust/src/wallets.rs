use std::collections::HashMap;
use crate::wallet::Wallet;

pub const WALLET_FILE: &str = "wallet.dat";

pub struct Wallets {
    wallets: HashMap<String, Wallet>,
}

impl Wallets {
    pub fn new() -> Wallets {
        todo!()
    }

    /// 创建一个钱包
    pub fn create_wallet(&mut self) -> String {
        todo!()
    }

    pub fn get_addresses(&self) -> Vec<String> {
        todo!()
    }

    /// 通过钱包地址查询钱包
    pub fn get_wallet(&self, address: &str) -> Option<&Wallet> {
        todo!()
    }

    /// 从本地文件加载钱包
    pub fn load_from_file(&mut self) {
        todo!()
    }
}