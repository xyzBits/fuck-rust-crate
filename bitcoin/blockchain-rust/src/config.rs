use std::collections::HashMap;
use std::env;
use std::sync::RwLock;

use once_cell::sync::Lazy;

pub static GLOBAL_CONFIG: Lazy<Config> = Lazy::new(|| Config::new());

/// 默认的节点地址
const DEFAULT_NODE_ADDR: &str = "127.0.0.1:2001";

const NODE_ADDRESS_KEY: &str = "NODE_ADDRESS";
const MINING_ADDRESS_KEY: &str = "MINING_ADDRESS";

/// Node 配置
pub struct Config {
    inner: RwLock<HashMap<String, String>>,
}

impl Config {
    pub fn new() -> Config {
        // 从环境变量获取节点地址
        let mut node_addr = String::from(DEFAULT_NODE_ADDR);
        if let Ok(addr) = env::var("NODE_ADDRESS") {
            node_addr = addr;
        }

        let mut map = HashMap::new();
        map.insert(String::from(NODE_ADDRESS_KEY), node_addr);

        Config {
            inner: RwLock::new(map),
        }
    }

    /// 获取节点地址
    pub fn get_node_addr(&self) -> String {
        let inner = self.inner.read().unwrap();
        inner.get(NODE_ADDRESS_KEY).unwrap().clone()
    }


    /// 设置矿工钱包地址
    pub fn set_mining_addr(&self, addr: String) {
        let mut inner = self.inner.write().unwrap();
        let _ = inner.insert(String::from(MINING_ADDRESS_KEY), addr);
    }

    /// 获取矿工钱包地址
    pub fn get_mining_addr(&self) -> Option<String> {
        let inner = self.inner.read().unwrap();
        if let Some(addr) = inner.get(MINING_ADDRESS_KEY) {
            return Some(addr.clone());
        }

        None
    }

    /// 检查矿工节点
    pub fn is_miner(&self) -> bool {
        let inner = self.inner.read().unwrap();
        inner.contains_key(MINING_ADDRESS_KEY)
    }
}


#[cfg(test)]
mod tests {
    use std::env;
    use crate::Config;
    use crate::config::{DEFAULT_NODE_ADDR, NODE_ADDRESS_KEY};

    #[test]
    fn test_env_var() {

        let key = "Path";
        match env::var(key) {
            Ok(val) => {println!("{key}: {val:?}")}
            Err(e) => {println!("couldn't interpret {key}: {e}")}
        }

    }


    #[test]
    fn test_new_config() {
        env::set_var(NODE_ADDRESS_KEY, DEFAULT_NODE_ADDR);

        let config = Config::new();
        let node_addr = config.get_node_addr();

        println!("{}", node_addr);
    }
}