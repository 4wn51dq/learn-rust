use std::sync::{Arc, Mutex};
use std::thread;
use tokio::time::{sleep, Duration};

mod block;
mod transaction;
mod chain;
mod hasher;

use block::{Block, Header};
use transaction::{Tx, TxStatus};
use chain::Blockchain;
use hasher::{Sha256, /*Keccak256*/ };

#[tokio::main]
async fn main() {
    let blockchain = Blockchain { blocks: vec![] };
    
    let block = Block::new(
        Header {
            previous_hash: None,
            merkle_root: [0; 32],
            nonce: 0,
            timestamp: 0,
        },
        vec![Tx {
            amount: 50,
            sender: [1; 20],
            receiver: [2; 20],
            status: TxStatus::Pending,
        }],
    ).unwrap();
    
    let shared_chain = Arc::new(Mutex::new(blockchain)); //putting blockchain on the heap with Arc<Mutex>>
    
    // we want to see how we can make two threads share the same blockchain.
    // so we put the blockchain in a safe mutation, then we allow shared ownership across threads.
    // now lives on the heap.

    // Arc clone would give the spawned thread its own ownership of shared data.
    let chain_clone = Arc::clone(&shared_chain);
    // move means take ownership of everything in this closure.
    let handle = thread::spawn(move || {
        // locking the mutex means only this thread can access blockchain rn, others wait
        let mut chain = chain_clone.lock().unwrap();
        let hasher = Sha256{};
        chain.add_new(block, &hasher).unwrap();
    });
    handle.join().unwrap(); // unlocks

    let chain = shared_chain.lock().unwrap();
    println!("{}", chain.blocks.len());

    let bc_ref: &Blockchain = &*chain;
    let json = serde_json::to_string_pretty(bc_ref).unwrap();
    println!("{}", json);
    
    println!("hexcode: {}", Block::merkle_root_hex(&bc_ref.blocks[0]));

    let start = std::time::Instant::now();

    /* let (b1, b2, b3) = tokio::join!(
        fetch_block_from_peer(1),
        fetch_block_from_peer(2),
        fetch_block_from_peer(3),
    ); 
    
    let b1 =fetch_block_from_peer(1).await;
    let b2 = fetch_block_from_peer(2).await;
    let b3 = fetch_block_from_peer(3).await; */

    fetch_fastest_block().await;

    println!("took {:?}", start.elapsed());
    
}

async fn fetch_block_from_peer(peer_id: u64) -> Block {
    println!("fetching block from peer: {}", peer_id);
    sleep(Duration::from_millis(100)).await;

    Block::new(
        Header{
            previous_hash: None,
            merkle_root: [peer_id as u8; 32],
            nonce: 0,
            timestamp: 0,
        },
        vec![
            Tx{
                amount: peer_id * 10,
                sender: [1; 20],
                receiver: [2; 20],
                status: TxStatus::Pending,
            }
        ]
    ).unwrap()
}

async fn fetch_fastest_block() -> Block {
    tokio::select! {
        b = fetch_block_from_peer(1) => { b },
        b = fetch_block_from_peer(2) => { b },
        b = fetch_block_from_peer(3) => { b },
    }
}
