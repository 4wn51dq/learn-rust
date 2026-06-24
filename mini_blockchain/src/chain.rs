use serde::{Deserialize, Serialize};
use crate::block::Block;
use crate::hasher::Hasher;
use crate::transaction::Tx;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Blockchain {
    pub blocks: Vec<Block>
}

impl Blockchain {
    pub fn add_new<H: Hasher>(&mut self, block: Block, hash_func: &H) -> Result<(), String> {
        if let Some(last) = self.blocks.last() {
            let previous_header_hash = hash_func.hash_header::<H>(&last.header)?;
            if block.header.previous_hash == Some(previous_header_hash) {
                self.blocks.push(block);
            } else {
                return Err(String::from("previous hash doesn't match"));
            }
        } else {
            self.blocks.push(block);
        }
        Ok(())
    }

    pub fn find_txs(&self, amount: u64) -> Vec<&Tx> {
        self.blocks.iter().flat_map(|block| block.txs.iter()).filter(|tx| tx.amount == amount).collect()
    }

    pub fn total_volume(&self) -> u64 {
        self.blocks.iter().flat_map(|block| block.txs.iter()).map(|tx| tx.amount).sum()
    }

    pub fn latest_block(&self) -> Option<&Block> {
        self.blocks.last()
    }

    pub fn block_index(&self) -> HashMap<[u8; 32], &Block> {
        let mut hashmap = HashMap::new();
        for block in &self.blocks {
            hashmap.insert(block.header.merkle_root, block);
        }
        hashmap
    }
}