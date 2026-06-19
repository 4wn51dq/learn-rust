fn main() {
    let hashers: Vec<Box<dyn Hasher>> = vec![
        Box::new(Sha256{}),
        Box::new(Keccak256{}),
    ];

    for hasher in &hashers {
        println!("{:?}", hasher.hash(&[1,2,3]));
    }
}

trait Hasher {
    fn hash(&self, data: &[u8]) -> [u8; 32];
}

struct Sha256 {

}

struct Keccak256 {

}

impl Hasher for Sha256 {
    fn hash(&self, data: &[u8]) -> [u8; 32] {
        [55; 32]
    }
}
impl Hasher for Keccak256 {
    fn hash(&self, data: &[u8]) -> [u8; 32] {
        [77; 32]
    }
}

fn compute_hash<T: Hasher>(hasher: T, data: &[u8]) -> [u8; 32] {
    hasher.hash(data)
}

#[derive(Debug)]
struct Block {
    header: Header,
    txs: Vec<Tx>,
}

#[derive(Debug)]
struct Header {
    previous_hash: [u8; 32],
    merkle_root: [u8; 32],
    version: u32,
    nonce: u64,
    timestamp: u64, 
}

#[derive(Debug)]
struct Tx {
    amount: u64,
    sender: [u8; 20],
    receiver: [u8; 20],
    status: TxStatus,
}

impl Tx {
    fn describe(&self) {
        match &self.status {
            TxStatus::Pending => println!("pending"),
            TxStatus::Confirmed (block_num) => println!("confirmed in block {}", block_num),
            TxStatus::Failed (reason) => println!("tx failed: {}", reason),
        }
    }
}

#[derive(Debug)]
enum TxStatus {
    Pending,
    Confirmed(u64), // block number
    Failed(String),
}

impl Block {
    fn new(header: Header, txs: Vec<Tx>) -> Result<Self, String> {
        if txs.is_empty() {
            return Err(String::from("no txs"))
        };
        let block = Block{
            header: header,
            txs: txs,
        };
        Ok(block)
    }
    fn tx_count(&self) -> usize {
        self.txs.len()
    }
}