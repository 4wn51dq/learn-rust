use serde::{Serialize, Deserialize};
use crate::transaction::Tx;
use crate::transaction;
pub use crate::errors::BlockchainError;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub header: Header,
    pub txs: Vec<Tx>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    pub previous_hash: Option<[u8; 32]>,
    pub merkle_root: [u8; 32],
    pub nonce: u64,
    pub timestamp: u64, 
}

impl Block {
    pub fn new(header: Header, txs: Vec<Tx>) -> Result<Self, BlockchainError> {
        if txs.is_empty() {
            return Err(BlockchainError::EmptyTransactions);
        };
        let mut block = Block{
            header,
            txs,
        };
        block.header.merkle_root = transaction::compute_merkle_root(&block.txs)?;
        Ok(block)
    }
    pub fn tx_count(&self) -> usize {
        self.txs.len()
    }

    pub fn merkle_root_hex(&self) -> String {
        format!("0x{}", hex::encode(self.header.merkle_root))
    }
}