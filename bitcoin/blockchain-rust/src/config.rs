use std::collections::HashMap;
use std::sync::RwLock;
use once_cell::sync::Lazy;

pub static GLOBAL_CONFIG: Lazy<Config> = Lazy::new(|| Config::new());

/// Node 配置
pub struct Config {
    inner: RwLock<HashMap<String, String>>,
}

impl Config {
    pub fn new() -> Config {
        todo!()
    }

    /// 获取节点地址
    pub fn get_node_addr(&self) -> String {
        todo!()
    }


    /// 设置矿工钱包地址
    pub fn set_mining_addr(&self, addr: String) {
        todo!()
    }
}