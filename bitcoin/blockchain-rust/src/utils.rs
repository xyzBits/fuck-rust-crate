use std::time::{SystemTime, UNIX_EPOCH};

use crypto::digest::Digest;
use ring::digest::{Context, SHA256};
use ring::rand::SystemRandom;
use ring::signature::{ECDSA_P256_SHA256_FIXED, ECDSA_P256_SHA256_FIXED_SIGNING, EcdsaKeyPair};

/// 获取当前时间戳，单位: ms
pub fn current_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as i64
}

/// 计算 sha256 哈希值
pub fn sha256_digest(data: &[u8]) -> Vec<u8> {
    let mut context = Context::new(&SHA256);
    context.update(data);
    let digest = context.finish();
    digest.as_ref().to_vec()
}

/// 计算 ripemd160 哈希值
pub fn ripemd160_digest(data: &[u8]) -> Vec<u8> {
    let mut ripemd160 = crypto::ripemd160::Ripemd160::new();
    ripemd160.input(data);

    let mut buf: Vec<_> = std::iter::repeat(0).take(ripemd160.output_bytes()).collect();

    ripemd160.result(&mut buf);
    buf
}

/// base58 编码
pub fn base58_encode(data: &[u8]) -> String {
    bs58::encode(data).into_string()
}

/// base58 解码
pub fn base58_decode(data: &str) -> Vec<u8> {
    bs58::decode(data).into_vec().unwrap()
}

/// 创建密钥对（椭圆曲线加密）
pub fn new_key_pair() -> Vec<u8> {
    let rng = SystemRandom::new();
    let pkcs8 = EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &rng).unwrap();

    pkcs8.as_ref().to_vec()
}

/// ECDSA P256 SHA256 签名
pub fn ecdsa_p256_sha256_sign_digest(pkcs8: &[u8], message: &[u8]) -> Vec<u8> {
    let key_pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, pkcs8).unwrap();
    let rng = SystemRandom::new();
    key_pair.sign(&rng, message).unwrap().as_ref().to_vec()
}

/// ECDSA P256 SHA256 签名验证
pub fn ecdsa_p256_sha256_sign_verify(public_key: &[u8], signature: &[u8], message: &[u8]) -> bool {
    let peer_public_key =
        ring::signature::UnparsedPublicKey::new(&ECDSA_P256_SHA256_FIXED, public_key);

    let result = peer_public_key.verify(message, signature.as_ref());
    result.is_ok()
}


#[cfg(test)]
mod tests {
    use data_encoding::{BASE64, HEXLOWER};
    use ring::signature::{ECDSA_P256_SHA256_FIXED_SIGNING, EcdsaKeyPair, KeyPair};

    use crate::utils::{base58_decode, base58_encode, ecdsa_p256_sha256_sign_digest, ecdsa_p256_sha256_sign_verify, new_key_pair, ripemd160_digest, sha256_digest};

    #[test]
    fn test_sha256_digest() {
        // sha256 会产生256位的哈希值，作为消息的摘要。这个摘要相当于一个32个字节的数组，通常有一个长度为64的16进制
        // 字符串表示，其中一个字节等于8位，一个16进制的字符长度为4位。
        // let digest = sha256_digest("hello".as_bytes());
        let digest = sha256_digest(b"hello");

        // 16进制编码输出
        // let hex_digest = HEXLOWER.encode(digest.as_slice());
        let hex_digest = HEXLOWER.encode(&digest);

        //2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824
        println!("SHA-256 digest is {}", hex_digest)
    }

    #[test]
    fn test_ripemd160() {
        // let bytes = ripemd160_digest("hello".as_bytes());
        let bytes = ripemd160_digest(b"hello");

        // let hex_str = HEXLOWER.encode(bytes.as_slice());
        let hex_str = HEXLOWER.encode(&bytes);


        // 108f07b8382412612c048d07d13f814118445acd
        println!("ripemd160 digest is {}", hex_str)
    }

    #[test]
    fn test_base58() {
        let sign = "hello world";
        let base58_sign = base58_encode(sign.as_bytes());

        let decode_bytes = base58_decode(base58_sign.as_str());


        let decode_str = String::from_utf8(decode_bytes).unwrap();
        assert_eq!(sign, decode_str.as_str());
    }

    #[test]
    fn test_ecdsa_sign_and_verify() {
        const MESSAGE: &[u8] = b"hello, world";
        let pkcs8 = new_key_pair();
        // 签名
        // let signature = ecdsa_p256_sha256_sign_digest(pkcs8.as_slice(), MESSAGE);
        let signature = ecdsa_p256_sha256_sign_digest(&pkcs8, MESSAGE);

        // 签名验证
        let key_pair =
            EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, pkcs8.as_slice()).unwrap();


        let public_key = key_pair.public_key().as_ref();

        let verify =
            ecdsa_p256_sha256_sign_verify(public_key, signature.as_slice(), MESSAGE);
        assert!(verify)
    }



    #[test]
    fn test_to_vec() {
        let data = [1, 2, 3, 4];

        // copy self into a new Vec
        let vec = data.to_vec();

        let data = b"Hello world";
        let result = data_encoding::BASE64.encode(data);

        let result1 = BASE64.decode(result.as_bytes()).unwrap();
        println!("{:?}", String::from_utf8(result1).unwrap());

    }
}











