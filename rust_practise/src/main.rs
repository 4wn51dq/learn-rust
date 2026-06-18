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
        status: TxStatus::Confirmed(123),
    };
    let txs: Vec<Tx> = vec![];
    let block = match Block::new(header, txs) {
        Ok(block) => block,
        Err(e) => panic!("error: {}", e),
    };
    block.txs[0].describe();
    
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