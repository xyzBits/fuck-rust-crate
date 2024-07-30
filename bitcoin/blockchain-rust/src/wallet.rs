use ring::signature::{ECDSA_P256_SHA256_FIXED_SIGNING, EcdsaKeyPair, KeyPair};
use serde::{Deserialize, Serialize};
use crate::utils::{base58_decode, base58_encode, new_key_pair, ripemd160_digest, sha256_digest};

const VERSION: u8 = 0x00;
pub const ADDRESS_CHECK_SUM_LEN: usize = 4;

#[derive(Clone, Serialize, Deserialize)]
pub struct Wallet {
    pkcs8: Vec<u8>,

    // 原生的公钥
    public_key: Vec<u8>,
}


impl Wallet {

    /// 创建一个钱包
    pub fn new() -> Self {
        let pkcs8 = new_key_pair();
        let key_pair =
            EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, pkcs8.as_ref()).unwrap();
        let public_key = key_pair.public_key().as_ref().to_vec();

        Self {
            pkcs8,
            public_key,
        }
    }


    /// 获取钱包地址
    /// 这里得到了一个真实的 BTC 地址，可以在 (Tokenview)[https://tokenview.com/cn/search/xxx] 查询它的余额
    /// 无论生成一个新的地址多少次，检查它的余额都是0，
    /// 这就是为什么选择一个合适的公钥加密算法是如此重要：
    /// 考虑到私钥是随机数，生成同一个数字的概率必须是尽可能地低，理想情况下，必须是低到永远不会重复
    /// 另外，你并不需要连接到一个比特币节点来获得一个地址，地址生成算法使用的多种开源算法可以通过很多编程语言和库实现
    ///
    ///
    /// 将一个公钥转换成一个 Base58 地址需要以下步骤：
    ///
    /// 1. 使用 RIPEMD160(SHA256(PubKey)) 哈希算法，取公钥并对其哈希两次
    ///
    /// 2. 给哈希加上地址生成算法版本的前缀
    ///
    /// 3. 对于第二步生成的结果，使用 SHA256(SHA256(payload)) 再哈希，计算校验和。校验和是结果哈希的前四个字节。
    ///
    /// 4. 将校验和附加到 version+PubKeyHash 的组合中。
    ///
    /// 5. 使用 Base58 对 version+PubKeyHash+checksum 组合进行编码。
    pub fn get_address(&self) -> String {

        // 1. 计算公钥的 SHA-256 哈希，然后使用 Ripemd-160 哈希
        let pub_key_hash = hash_pub_key(&self.public_key);

        let mut payload = vec![];

        //2. 添加版本字节，通常为 0x00，用于标识地址类型，代表比特币主网络的地址类型，通过这个来区分不同类型的地址
        payload.push(VERSION);
        payload.extend(pub_key_hash);

        // 3. 计算校验和，是为了防止地址在传输过程中的误输入，校验和 = SHA256(SHA256(payload))
        let checksum = checksum(&payload);

        payload.extend(&checksum);

        // version + pub_key_hash + checksum
        // 4. 拼接并进行Base58 编码，将版本字节，地址哈希和校验和拼接在一起，然后使用
        //    Base58 编码转换为一串字符，形成最终的比特币地址
        base58_encode(&payload)

    }
}



/// 计算公钥哈希
/// 计算公钥的 SHA-256 哈希，然后使用 Ripemd-160 哈希
/// 这样可以缩短哈希长度，保持足够的安全性
pub fn hash_pub_key(pub_key: &[u8]) -> Vec<u8> {
    let pub_key_sha256 = sha256_digest(pub_key);
    // ripemd160_digest(pub_key_sha256.as_slice());
    ripemd160_digest(&pub_key_sha256)
}


/// 计算校验和
/// 计算校验和是为了防止地址在传输过程中的误输入，通过对地址哈希加上版本字节结果的两个
/// SHA-256 哈希运算，并取前4个字节作为校验和
fn checksum(payload: &[u8]) -> Vec<u8> {
    let first_sha = sha256_digest(payload);
    let second_sha = sha256_digest(first_sha.as_slice());
    second_sha[0..ADDRESS_CHECK_SUM_LEN].to_vec()
}


/// 验证地址有效
/// 根据地址的字节结构校验地址
pub fn validate_address(address: &str) -> bool {
    let payload = base58_decode(address);

    let actual_checksum = payload[payload.len() - ADDRESS_CHECK_SUM_LEN..].to_vec();
    let version = payload[0];

    let pub_key_hash = payload[1..payload.len() - ADDRESS_CHECK_SUM_LEN].to_vec();

    let mut target_vec = vec![];
    target_vec.push(version);
    target_vec.extend(pub_key_hash);

    let target_checksum = checksum(&target_vec);
    actual_checksum.eq(&target_checksum)

}


/// 通过公钥哈希计算地址
pub fn convert_address(pub_hash_key: &[u8]) -> String {
    let mut payload = vec![];
    payload.push(VERSION);
    payload.extend(pub_hash_key);

    let checksum = checksum(&payload);
    payload.extend(checksum);

    base58_encode(&payload)
}



#[cfg(test)]
mod tests {
    use crate::utils::{ecdsa_p256_sha256_sign_digest, ecdsa_p256_sha256_sign_verify};
    use crate::wallet::{validate_address, Wallet};

    #[test]
    fn test_new_wallet() {
        let address = Wallet::new().get_address();
        println!("The address is {}", address);
    }

    #[test]
    pub fn test_validate_address() {
        // BTC 创世块：1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa
        let valid = validate_address("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa");
        assert!(valid);
    }


    #[test]
    fn test_create_address_and_validate() {
        let address = Wallet::new().get_address();
        println!("address: {}", address);

        let valid = validate_address(&address);
        println!("valid: {}", valid);
    }


    #[test]
    fn test_sign_and_verify() {
        let wallet = Wallet::new();
        let secret_key = wallet.pkcs8;
        let public_key = wallet.public_key;

        let message = b"hello world";
        let signature = ecdsa_p256_sha256_sign_digest(&secret_key, message);

        let verified = ecdsa_p256_sha256_sign_verify(&public_key, &signature, message);

        println!("{}", verified);

    }
}



















