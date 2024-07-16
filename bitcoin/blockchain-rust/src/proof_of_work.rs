use std::ops::ShlAssign;
use data_encoding::HEXLOWER;
use num_bigint::{BigInt, Sign};
use std::borrow::Borrow;

use crate::block::Block;
use crate::utils::sha256_digest;

pub struct ProofOfWork {
    block: Block,
    target: BigInt,
}

/// 难度值，这里表示哈希的前20位必须是0
const TARGET_BITS: i32 = 8;

/// 限制 nonce 避免整型溢出
const MAX_NONCE: i64 = i64::MAX;

impl ProofOfWork {
    pub fn new_proof_of_work(block: Block) -> ProofOfWork {
        let mut target = BigInt::from(1);

        // target 等于 1 左移 256 位 - TARGET_BITS 位
        target.shl_assign(256 - TARGET_BITS);
        ProofOfWork {
            block,
            target,
        }
    }

    /// 工作量证明用到的数据
    fn prepare_data(&self, nonce: i64) -> Vec<u8> {
        let pre_block_hash = self.block.get_pre_block_hash();
        let transactions_hash = self.block.hash_transactions();
        let timestamp = self.block.get_timestamp();

        let mut data_bytes = vec![];

        data_bytes.extend(pre_block_hash.as_bytes());
        data_bytes.extend(transactions_hash);
        data_bytes.extend(timestamp.to_be_bytes());
        data_bytes.extend(TARGET_BITS.to_be_bytes());
        data_bytes.extend(nonce.to_be_bytes());

        data_bytes
    }

    /// 工作量的证明就是寻找有效的哈希
    pub fn run(&self) -> (i64, String) {
        let mut nonce = 0;
        let mut hash = Vec::new();

        println!("Mining the block");

        while nonce < MAX_NONCE {
            let data = self.prepare_data(nonce);
            let hash = sha256_digest(data.as_slice());
            let hash_int = BigInt::from_bytes_be(Sign::Plus, hash.as_slice());

            // 1. 在比特币中，当一个块被挖出来以后， target bits 代表了区块头里存储的难度，也就是开头有多少个 0
            // 2. 这是的 20 指的是算出来的哈希前 20 必须是 0 ，如果用 16 进制表示，就是前5位必须是 0， 这一点从
            // 最后的输出可以看出来
            //  例如，target 16 进制输出 是 000010000000000000000000000000000000000000000000000000000000
            //  目前我们并不会实现一个动态调整目标的算法，所以将难度定义成一个全局的常量即可
            // 3. 将哈希与目标数 target 进行比较，先把 哈希转成一个大整数，然后检查它是否小于目标，小就是有效，反之无效
            if hash_int.lt(self.target.borrow()) {
                println!("{}", HEXLOWER.encode(hash.as_slice()));
                break;
            } else {
                nonce += 1;
            }
        }


        (nonce, HEXLOWER.encode(hash.as_slice()))
    }
}

#[cfg(test)]
mod tests {
    use std::ops::ShlAssign;
    use data_encoding::HEXLOWER;
    use num_bigint::{BigInt, Sign, ToBigInt, ToBigUint};
    use crate::proof_of_work::TARGET_BITS;

    #[test]
    fn test_to_bytes() {
        let i = -1125.to_bigint().unwrap();
        assert_eq!(i.to_bytes_be(), (Sign::Minus, vec![4, 101]));

        let result = 1.to_bigint().unwrap() << (256 - TARGET_BITS);

        println!("{}", result);
    }

    #[test]
    fn test_bigint_from_bytes() {
        let a = BigInt::from(256);
        let (s, vec) = a.to_bytes_be();
        println!("s = {:?}", s);
        println!("vec = {:?}", vec);

        // big-endian
        let b = BigInt::from_signed_bytes_be(vec.as_slice());
        println!("{}", b);
    }

    #[test]
    fn test_target_bits() {
        let mut target = BigInt::from(1);
        target.shl_assign(256 - TARGET_BITS);
        // target <<= 256 - TARGET_BITS;
        println!("{}", target); // output: 6901746346790563787434755862277025452451108972170386555162524223799296

        // 16进制输出, 大端序
        let (_, vec) = target.to_bytes_be();
        let target_hex = HEXLOWER.encode(vec.as_slice());
        println!("{}", target_hex) // output: 100000000000000000000000000000000000000000000000000000000000
    }
}