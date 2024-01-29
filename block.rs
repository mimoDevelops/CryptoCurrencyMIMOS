// block.rs
use crate::transaction::Transaction;
use sha2::{Sha256, Digest};
use std::fmt::{self, Formatter, Debug}; // Import necessary traits

#[derive(Serialize, Deserialize, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub hash: String,
    pub prev_block_hash: String,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(index: u64, timestamp: i64, prev_block_hash: String, transactions: Vec<Transaction>) -> Self {
        Block {
            index,
            timestamp,
            hash: String::new(),
            prev_block_hash,
            nonce: 0,
            transactions,
        }
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(&serde_json::to_string(&self).unwrap());
        format!("{:x}", hasher.finalize())
    }

    pub fn set_hash(&mut self) {
        self.hash = self.calculate_hash();
    }
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Block {{ index: {}, timestamp: {}, hash: {}, prev_block_hash: {}, nonce: {}, transactions: {:?} }}",
               self.index, self.timestamp, self.hash, self.prev_block_hash, self.nonce, self.transactions)
    }
}