use std::collections::HashMap;

use bincode::ErrorKind::SequenceMustHaveLength;
use bitcoincash_addr::{Address, HashType, Scheme};
use crypto::digest::Digest;
use crypto::ed25519;
use crypto::ripemd160::Ripemd160;
use crypto::sha2::Sha256;
use log::info;
use rand::RngCore;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};

use crate::errors::Result;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Wallet {
    pub secret_key: Vec<u8>,
    pub public_key: Vec<u8>,

}


impl Wallet {
    /// 首先生成一个随机的 32 字节的密钥，也就是256位，
    /// 然后使用这个密钥生成一个 ed25519 的公钥/私钥对
    /// 然后将这个公钥/私钥对存储在 Wallet 结构体中
    fn new() -> Self {
        let mut key = [0u8; 32];
        let mut rand = OsRng::default();
        rand.fill_bytes(&mut key);
        let (secret_key, public_key) = ed25519::keypair(&key);
        let secret_key = secret_key.to_vec();
        let public_key = public_key.to_vec();

        Wallet {
            secret_key,
            public_key,
        }
    }

    /// GetAddress returns the wallet address
    /// 首先将公钥进行哈希计算，然后将哈希后的结果编码为一个比特币地址
    pub fn get_address(&self) -> String {
        let mut pub_hash = self.public_key.clone();

        // 先 sha256 ，再 ripemd160 取 20字节
        hash_pub_key(&mut pub_hash);
        let address = Address {
            body: pub_hash,
            scheme: Scheme::Base58, // 使用 base 58 编码方案
            hash_type: HashType::Script, // 该地址对应的是一个脚本而非单一的公钥
            ..Default::default()
        }; // 这个结构体用于表示一个比特币地址

        address.encode().unwrap()
    }
}

/// hash pub key hashes public key
/// 对公钥进行哈希计算，它首先使用 sha256 算法对公钥进行哈希，然后再使用 ripemd160 算法对结果进行哈希
/// 得到的结果是一个 20 字节的哈希值
pub fn hash_pub_key(pub_key: &mut Vec<u8>) {
    let mut hasher1 = Sha256::new();
    hasher1.input(pub_key);
    hasher1.result(pub_key);

    let mut hasher2 = Ripemd160::new();
    hasher2.input(pub_key);
    pub_key.resize(20, 0);
    hasher2.result(pub_key);
}


pub struct Wallets {
    wallets: HashMap<String, Wallet>,
}


impl Wallets {
    /// NewWallets creates Wallets and fills it from a file if it exists
    /// 从 db 中读取钱包数据，创建一个 Wallets HashMap
    pub fn new() -> Result<Wallets> {
        let mut wlt = Wallets {
            wallets: HashMap::<String, Wallet>::new(),
        };

        let db = sled::open("data/wallets")?;

        for item in db.into_iter() {
            let item = item?;
            let address = String::from_utf8(item.0.to_vec())?;
            let wallet: Wallet = bincode::deserialize(&item.1.to_vec())?;
            wlt.wallets.insert(address, wallet);
        }

        // 虽然 db 会在作用域结束后自动 drop ，但这里希望显式地立即释放
        drop(db);

        Ok(wlt)
    }


    /// CreateWallet adds a wallet to Wallets
    pub fn create_wallet(&mut self) -> String {
        let wallet = Wallet::new();
        let address = wallet.get_address();
        self.wallets.insert(address.clone(), wallet);
        info!("create wallet: {}", address);
        address
    }


    /// GetAddresses returns an array of addresses stored in the wallet file
    pub fn get_all_addresses(&self) -> Vec<String> {
        let mut addresses = Vec::new();

        for (address, _) in &self.wallets {
            addresses.push(address.clone());
        }
        addresses
    }

    /// GetWallet return a wallet by its address
    pub fn get_wallet(&self, address: &str) -> Option<&Wallet> {
        self.wallets.get(address)
    }


    pub fn save_all(&self) -> Result<()> {
        let db = sled::open("data/wallets")?;

        for (address, wallet) in &self.wallets {
            let data = bincode::serialize(wallet)?;
            db.insert(address, data)?;
        }

        db.flush()?;
        drop(db);

        Ok(())
    }
}









