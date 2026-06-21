use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tx {
    pub amount: u64,
    pub sender: [u8; 20],
    pub receiver: [u8; 20],
    pub status: TxStatus,
}

impl Tx {
    pub fn describe(&self) {
        match &self.status {
            TxStatus::Pending => println!("pending"),
            TxStatus::Confirmed (block_num) => println!("confirmed in block {}", block_num),
            TxStatus::Failed (reason) => println!("tx failed: {}", reason),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TxStatus {
    Pending,
    Confirmed(u64), // block number
    Failed(String),
}