use crate::block::Header;
use sha2::{Sha256, Digest};
use crate::transaction::{Tx};
pub use crate::errors::BlockchainError;
pub trait Hasher {
    fn hash(&self, data: &[u8]) -> Result<[u8; 32], BlockchainError>;
    fn hash_header(&self, data: &Header) -> Result<[u8; 32], BlockchainError> {
        let header_bytes = serde_json::to_vec(data)
            .map_err(|e| BlockchainError::SerializationError(e.to_string()))?;
        self.hash(&header_bytes)
    }
    fn hash_tx(&self, tx: &Tx) -> Result<[u8; 32], BlockchainError> {
        let tx_bytes = serde_json::to_vec(tx)
            .map_err(|e| BlockchainError::SerializationError(e.to_string()))?;
        self.hash(&tx_bytes)
    }
}
pub struct SHA256 {}

pub struct Keccak256 {}

impl Hasher for SHA256 {
    fn hash(&self, data: &[u8]) -> Result<[u8; 32], BlockchainError> {
        let result = Sha256::digest(data);
        
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        
        Ok(hash)
    }
}

pub fn compute_hash<T: Hasher>(hasher: T, data: &[u8]) -> Result<[u8; 32], BlockchainError> {
    hasher.hash(data)
}