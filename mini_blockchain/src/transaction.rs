use serde::{Serialize, Deserialize};
pub use crate::errors::BlockchainError;
use crate::hasher::{Hasher, SHA256};


#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub fn hashed_tx(&self, hasher: &dyn Hasher) -> Result<[u8; 32], BlockchainError>{
        hasher.hash_tx(&self).map_err(|e| BlockchainError::EmptyTransactions)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TxStatus {
    Pending,
    Confirmed(u64), // block number
    Failed(String),
}

pub fn compute_merkle_root(txs: &[Tx]) -> Result<[u8; 32], BlockchainError> {
    let hasher = SHA256{};
    if txs.is_empty() {
        return Err(BlockchainError::EmptyTransactions)
    } else if txs.len() == 1 {
        return Tx::hashed_tx(&txs[0], &hasher)
    } else {
        let mut current_level: Vec<[u8; 32]> = txs.iter().map(
            |t| hasher.hash_tx(t).map_err(|e| BlockchainError::HashingError)
        ).collect::<Result<Vec<_>, _>>()?;

        while current_level.len()>1 {
            let mut next_level: Vec<[u8; 32]> = vec![];

            for chunk in current_level.chunks(2) {
                let combined = if chunk.len() == 2 {
                    chunk.concat()
                } else {
                    chunk.repeat(2).concat()
                };
                next_level.push(hasher.hash(&combined)?);
            }
            current_level = next_level;
        }
        Ok(current_level[0])
    }
}