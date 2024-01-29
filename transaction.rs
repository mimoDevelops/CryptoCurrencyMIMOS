// transaction.rs
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f32,
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: f32) -> Transaction {
        Transaction { sender, receiver, amount }
    }

    pub fn genesis() -> Transaction {
        Transaction::new("Genesis".to_owned(), "Genesis".to_owned(), 0.0)
    }
}