use crate::block::Header;
pub trait Hasher {
    fn hash(&self, data: &[u8]) -> [u8; 32];
    fn hash_header(&self, data: &Header) -> [u8; 32] {
        let header_bytes = serde_json::to_vec(data).unwrap();
        self.hash(&header_bytes)
    }
}
pub struct Sha256 {

}
pub struct Keccak256 {

}

impl Hasher for Sha256 {
    fn hash(&self, data: &[u8]) -> [u8; 32] {
        [55; 32]
    }
}impl Hasher for Keccak256 {
    fn hash(&self, data: &[u8]) -> [u8; 32] {
        [77; 32]
    }
}

pub fn compute_hash<T: Hasher>(hasher: T, data: &[u8]) -> [u8; 32] {
    hasher.hash(data)
}