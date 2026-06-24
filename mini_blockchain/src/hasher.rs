use crate::block::Header;
use sha2::{Sha256, Digest};
use hex::{FromHex, FromHexError};
pub trait Hasher {
    fn hash(&self, data: &[u8]) -> Result<[u8; 32], String>;
    fn hash_header<T: Hasher>(&self, data: &Header) -> Result<[u8; 32], String> {
        let header_bytes = serde_json::to_vec(data).unwrap();
        self.hash(&header_bytes)
    }
}
pub struct SHA256 {}

pub struct Keccak256 {}

impl Hasher for SHA256 {
    fn hash(&self, data: &[u8]) -> Result<[u8; 32], String> {
        let hashed = FromHex::from_hex(Sha256::digest(data)).map_err(
            |e: FromHexError| e.to_string()
        )?;
        Ok(hashed)
    }
}

pub fn compute_hash<T: Hasher>(hasher: T, data: &[u8]) -> Result<[u8; 32], String> {
    hasher.hash(data)
}