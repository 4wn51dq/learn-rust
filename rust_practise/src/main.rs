fn main() {
    let header = Header {
        previous_hash: [0; 32],
        merkle_root: [1; 32],
        version: 2,
        nonce: 12345678987654321,
        timestamp: 193746894,
    };
    let tx = Tx {
        amount: 123,
        sender: [66; 20],
        receiver: [77; 20],
    };
    let txs: Vec<Tx> = vec![tx];
    let block = Block::new(header, txs);
    println!("the block {:?} contains {} transactions", block, block.tx_count());
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
}

enum TxStatus {
    Pending,
    Confirmed(u64), // block number
    Failed(str)
}

impl Block {
    fn new(header: Header, txs: Vec<Tx>) -> Self {
        let block = Block{
            header: header,
            txs: txs,
        };
        block
    }
    fn tx_count(&self) -> usize {
        self.txs.len()
    }
}