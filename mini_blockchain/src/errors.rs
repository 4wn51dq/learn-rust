use thiserror::Error;

#[derive(Debug, Error)]
 pub enum BlockchainError {
    #[error("genesis block cannot have a previous hash")]
    InvalidGenesis,
    
    #[error("previous hash doesn't match")]
    InvalidPreviousHash,
    
    #[error("block must contain at least one transaction")]
    EmptyTransactions,
    
    #[error("serialization failed: {0}")]
    SerializationError(String),
 }