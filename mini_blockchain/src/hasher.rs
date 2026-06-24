use crate::block::Header;
use sha2::{Sha256, Digest};
use hex::{FromHex, FromHexError};
pub trait Hasher {
    fn hash(&self, data: &[u8]) -> Result<[u8; 32], String>;
    fn hash_header(&self, data: &Header) -> Result<[u8; 32], String> {
        let header_bytes = serde_json::to_vec(data).unwrap();
        self.hash(&header_bytes)
    }
}
pub struct SHA256 {}

pub struct Keccak256 {}

impl Hasher for SHA256 {
    fn hash(&self, data: &[u8]) -> Result<[u8; 32], String> {
        let result = Sha256::digest(data);
        
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        
        Ok(hash)
    }
}

pub fn compute_hash<T: Hasher>(hasher: T, data: &[u8]) -> Result<[u8; 32], String> {
    hasher.hash(data)
}