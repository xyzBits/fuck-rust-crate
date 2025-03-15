use alloy_primitives::{Address, keccak256};
use k256::{ecdsa::SigningKey, elliptic_curve::sec1::ToEncodedPoint};
use rand::rngs::OsRng;

fn main() {
    // 1. 生成私钥
    let signing_key = SigningKey::random(&mut OsRng);
    let private_key = hex::encode(signing_key.to_bytes());
    println!("Private Key: 0x{}", private_key);

    // 2. 推导公钥
    let public_key = signing_key.verifying_key().to_encoded_point(false);
    let raw_public_key = &public_key.as_bytes()[1..]; // 去掉04前缀
    println!("Public Key: 0x{}", hex::encode(raw_public_key));

    // 3. 生成地址
    let hash = keccak256(raw_public_key);
    let address = Address::from_slice(&hash[12..]);
    println!("Address: {}", address.to_checksum(None));
}
