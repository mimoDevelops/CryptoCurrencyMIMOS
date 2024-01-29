// proof_of_work.rs
use crate::block::Block;

pub struct ProofOfWork {
    pub block: Block,
    pub difficulty: usize,
}

impl ProofOfWork {
    pub fn new(block: Block) -> Self {
        ProofOfWork {
            block,
            difficulty: 2, // number of leading zeroes in hash
        }
    }

    pub fn mine(&mut self) -> Block {
        loop {
            self.block.nonce += 1;
            let hash = self.block.calculate_hash();
            if self.is_valid_hash(&hash) {
                self.block.hash = hash;
                return self.block.clone();
            }
        }
    }

    fn is_valid_hash(&self, hash: &str) -> bool {
        hash.starts_with(&"0".repeat(self.difficulty))
    }
}
