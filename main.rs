#[macro_use]
extern crate serde;

// main.rs
mod block;
mod blockchain;
mod proof_of_work;
mod transaction;
#[macro_use]
extern crate serde_derive;

use blockchain::Blockchain;
use transaction::Transaction;

fn main() {
    let mut chain = Blockchain::new();

    // Add the first block following the genesis block
    let transactions1 = create_sample_transactions();
    chain.add_block(transactions1);

    // Add a second block
    let transactions2 = create_sample_transactions();
    chain.add_block(transactions2);

    // Add a third block
    let transactions3 = create_sample_transactions();
    chain.add_block(transactions3);

    // Add as many blocks as you need here...

    // Print out each block in the blockchain
    for block in chain.blocks {
        println!("{:?}", block);
    }
}

fn create_sample_transactions() -> Vec<Transaction> {
    vec![
        Transaction::new("Dave".to_owned(), "Eve".to_owned(), 75.0),
        Transaction::new("Eve".to_owned(), "Frank".to_owned(), 120.0),
        Transaction::new("Frank".to_owned(), "Grace".to_owned(), 50.0),
    ]
}
