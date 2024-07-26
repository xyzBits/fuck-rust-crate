
use ed25519_dalek::Signer;
use rand::rngs::OsRng;
use ripemd::{Digest, Ripemd160};
use sha256::Sha256Digest;

const VERSION: u8 = 0x00;
pub const ADDRESS_CHECK_SUM_LEN: usize = 4;
pub type PublicKey = [u8; 32];
pub type PrivateKey = [u8; 32];
pub type Address = String;
pub type Ripemd160Hash = [u8; 20];

#[derive(Debug)]
pub struct Keypair(ed25519_dalek::SigningKey);

impl Keypair {
    pub fn new() -> Self {
        Self(ed25519_dalek::SigningKey::generate(&mut OsRng))
    }

    pub fn public_key(&self) -> PublicKey {
        self.0.verifying_key().to_bytes()
    }

    pub fn private_key(&self) -> PrivateKey {
        self.0.to_bytes()
    }

    pub fn address(&self) -> String {
        let mut hash_160 = Self::pub_key_hash(self).to_vec();
        // 步骤2， 添加版本字节，版本字节用于标识地址类型，version 是一个常量，代表比特币
        // 主网的地址类型，其值通常为 0x00 ，通过在哈希值前添加这个版本字节，
        // 能够区别不同类型的比特币地址
        hash_160.insert(0, VERSION);
        let rt = [hash_160.clone(), checksum(hash_160.as_slice())].concat();

        // 拼接并进行 Base58 编码，
        // 最后一点是将版本字节，地址哈希 校验和拼接在一起，
        // 然后使用 Base58 编码转换为一串字符串，
        // 形成最终的比特币地址
        // base58 函数将拼接后的结果转换为 Base58 编码的字符串
        // 这是比特币地址的标准格式，易于人类阅读且避免了容易混淆的字符
        base58_encode(rt.as_slice())
    }

    /// 步骤1：计算公钥的 SHA-256 哈希，然后再使用 RIPEMD-160 哈希
    /// 这样可以缩短 哈希的长度，保证足够的安全性
    pub fn pub_key_hash(&self) -> Ripemd160Hash {
        // digest 方法对公钥进行 SHA-256 哈希运算
        let hash = self.public_key().digest();
        // 然后调用 ripemd160_digest 函数对结果进行 RIPEMD-160 哈希运算
        ripemd160_digest(hash.as_bytes())
    }

    pub fn prikey_hex(&self) -> String {
        hex::encode(self.private_key())
    }

    pub fn pubkey_hex(&self) -> String {
        hex::encode(self.public_key())
    }
    // pub fn sign(&self, msg: &[u8]) -> Signature {
    //     self.0.sign(msg).into()
    // }
    //
    // pub fn verify(&self, message: &[u8], signature: Signature) -> bool {
    //     self.0.verify(message, &signature.into()).is_ok()
    // }

    pub fn from_bytes(private_key: &PrivateKey) -> Self {
        Self(ed25519_dalek::SigningKey::from_bytes(private_key))
    }
}
///用地址对比公钥哈希
/// 在验证交易的时候，只知道一个地址，比特币因为其签名算法，
/// 可以从地址获得公钥哈希，所以是通过这个原理来验证签名交易
///
pub fn address_verify(addr: &Address, pubkeyhash: Ripemd160Hash) -> bool {
    // 首先从 address 使用 base58_decode 来获得原始地址
    let payload = base58_decode(addr);

    // 再通过去除开头的版本信息的结尾的校验和，取中里的公钥哈希进行对比
    let pub_key_hash: Ripemd160Hash = payload[1..payload.len() - ADDRESS_CHECK_SUM_LEN]
        .try_into()
        .unwrap();
    pubkeyhash.eq(&pub_key_hash)
}

pub fn ripemd160_digest(data: &[u8]) -> Ripemd160Hash {
    let mut ripemd160 = Ripemd160::new();
    ripemd160.update(data);
    let ret = ripemd160.finalize();
    ret[..].try_into().unwrap()
}

pub fn base58_encode(data: &[u8]) -> String {
    bs58::encode(data).into_string()
}

pub fn base58_decode(data: &str) -> Vec<u8> {
    bs58::decode(data).into_vec().unwrap()
}

/// 计算校验和
/// 计算校验和是为了防止地址在传输过程中的误输入，通过对地址哈希加上版本字节的结果
/// 作两个 SHA-256 哈希运算，并取前4个字节作为校验和
fn checksum(payload: &[u8]) -> Vec<u8> {
    let first_sha = payload.digest();
    let second_sha = first_sha.digest();
    second_sha[0..ADDRESS_CHECK_SUM_LEN].as_bytes().to_vec()
}

#[test]
fn test() {
    use crate::account::Account;
    let account = Account::new();
    let _private_key = account.private_key();
    let _public_key = account.public_key();
    let pub_key_hash = account.pub_key_hash();
    let address = account.address();
    let ret = address_verify(&address, pub_key_hash);
    dbg!(ret);
}