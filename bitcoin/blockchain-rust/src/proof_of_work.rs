use num_bigint::BigInt;
use crate::block::Block;

pub struct ProofOfWork {
    block: Block,
    target: BigInt,
}

impl ProofOfWork {
    pub fn new_proof_of_work(block: Block) -> ProofOfWork {
        todo!()
    }

    pub fn run(&self) -> (i64, String) {
        todo!()
    }
}