// blockchain.rs
use crate::block::Block;
use crate::proof_of_work::ProofOfWork;
use crate::transaction::Transaction;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut bc = Blockchain { blocks: Vec::new() };
        bc.create_genesis_block();
        bc
    }

    pub fn create_genesis_block(&mut self) {
        let genesis_block = Block::new(0, current_time(), String::new(), vec![Transaction::genesis()]);
        self.blocks.push(genesis_block);
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let last_block = self.blocks.last().unwrap();
        let new_block = Block::new(
            last_block.index + 1,
            current_time(),
            last_block.hash.clone(),
            transactions,
        );

        let mut proof = ProofOfWork::new(new_block);
        let mined_block = proof.mine();
        self.blocks.push(mined_block);
    }
}

fn current_time() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let start = SystemTime::now();
    start.duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs() as i64
}
