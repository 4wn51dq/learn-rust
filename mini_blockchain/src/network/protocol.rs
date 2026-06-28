use serde::{Deserialize, Serialize};
use crate::block::Block;


#[derive(Serialize, Deserialize)]
pub enum Message {
    Handshake{
        chain_id: u32,
        chain_height: u64,
        best_hash: [u8; 32],
    }, 
    Ping,
    Pong, // response to Ping
    NewBlock([u8; 32]), // propagates hash 
    BlockRequest(u64), // propagates block number
    Block(Block), // response to Block Request
}

pub fn encode(message: &Message) -> Result<Vec<u8>, bincode::Error> {
    bincode::serialize(message)
}

pub fn decode(data: &[u8]) -> Result<Message, bincode::Error> {
    bincode::deserialize(data)
}
