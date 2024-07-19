use std::ops::ShlAssign;
use data_encoding::HEXLOWER;
use num_bigint::{BigInt, Sign};
use std::borrow::Borrow;

use crate::block::Block;
use crate::utils::sha256_digest;


/// 工作量证明，区块链的一个关键点是，一个人必须经过一系列困难的工作，才能将数据放入到区块链中
/// 正是由于这种困难的工作，才保证了区块链的安全和一致
/// 此外，完成这个工作的人，也会获得相应奖励
///
/// 这个机制与生活现象非常类似，一个人必须通过努力工作，才能获得回报或者资历，用以支撑他们的生活，
/// 在区块链中，是通过网络中的参与者不断的工作来支撑起整个网络，矿工不断地向区块链中加入新块，然后获得相应
/// 的奖励，在这种机制下，新生成的区块能够被安全地加入到这个区块链中，它维护整个区块链数据库的稳定性，
/// 完成这个工作的人必须要证明一点，即他必须要证明他的确完成了这些工作 ，
///
/// 整个努力工作并进行证明的机制，就叫做工作量证明，要想完成工作非常地不容易，
/// 因为这需要大量的计算能力，即使是高性能计算机，也无法在短时间内快速完成，另外，
/// 这个工作的困难会随着时间不断增长，
/// 以保持10分钟出一个新块的速度，
/// 在比特币中，这个工作就是找到一个块的哈希，同时这个哈希满足了一些必要条件，
/// 也就是充当了证明的角色，因此，寻求证明，有效的哈希，就是矿工要做的事情
///
///
/// 1。 无法从一个哈希值恢复原始数据，也就是说，哗然并不是加密
/// 2。 对于特定的数据，只能有一个哈希，并且这个哈希是唯一的
/// 3。 即使是仅仅改变输入数据中的一个字节，也会导致输出一个完全不同的哈希
///
///
///
/// 1。 取一些公开的数据，比如email的话，就是接收者的邮件地址，在比特币中，它是区块头
/// 2。 给这个公开数据加一个计数器，计数器默认从0开始
/// 3。 将data 数据 和 计数器 counter 组合到一起，获得一个哈希
/// 4。 检查哈希是否符合一定的条件
///     如果符合条件，结束 ，
///     如果不符合，增加计数器，重复步骤3-4
///
/// 因此，这是一个暴力算法，改变计算器，计算新的哈希，检查 ，增加计算器，计算哗然，检查 ，如此往复
/// 这也是为什么说它的计算成本很高，因为这一步需要如此反复不断地计算和检查
///
/// 一个哈希要满足的必要条件，在原始的 hashCash 实现中，它的要求是，一个哈希的前20位必须是0
/// 在比特币中，这个要求会随着时间而不断变化 ，因为按照设计，必须保诚每10分钟生成一个块，而不论计算能力会随着
/// 时间增加，或者是会有越来越多的矿工加入网络，所以需要动态调整这个必要条件
///
/// data = I like donuts，
///找到一个前3个字节全是0的哈希
///
pub struct ProofOfWork {
    block: Block,
    target: BigInt,
}

/// 难度值，这里表示哈希的前20位必须是0
/// 挖矿难度值 ，
/// 在比特币中，当一个块被挖出来 后，target bits 代表了区块里头存储的难度，也就是开头有多少个0，
/// 这里的24指的是算出来 的哈希前24位必须 是0，如果用16进制表示 ，就是前6位必须 是0
/// 目前不会实现动态调整的自满，所以定义为常量
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