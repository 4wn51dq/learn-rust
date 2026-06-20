

fn main() {
     /* let genesis_header = Header {
        previous_hash: None,
        merkle_root: [0; 32],
        nonce: 0000000000000,
        timestamp: 000000000000,
    };
    let tx1 = Tx{
        amount: 1,
        sender: [6; 20],
        receiver: [7; 20],
        status: TxStatus::Confirmed(1),
    };
    let tx2 = Tx{
        amount: 1,
        sender: [6; 20],
        receiver: [9; 20],
        status: TxStatus::Confirmed(2),
    };
    let header2 = Header {
        previous_hash: Some(genesis_header.merkle_root),
        merkle_root: [1; 32],
        nonce: 111111111111,
        timestamp: 000000023444,
    };
    let genesis = Block {
        header: genesis_header,
        txs: vec![tx1],
    };
    let block2 = Block {
        header: header2,
        txs: vec![tx2]
    };
    let mut blockchain = Blockchain {
        blocks: vec![]
    };
    blockchain.add_new(genesis);
    blockchain.add_new(block2);
    let results = blockchain.find_txs(1);
    for tx in results {
        tx.describe();
    }
    */

    let mut blockchain = Blockchain { blocks: vec![] };
    
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
    
    blockchain.add_new(block);
    
    let results = blockchain.find_txs(50);
    println!("{:?}", results);

    let prev_hash = blockchain.blocks[0].header.merkle_root;
    
    blockchain.add_new(Block::new(
        Header {
            previous_hash: Some(prev_hash),
            merkle_root: [1; 32],
            nonce: 1,
            timestamp: 1,
        },
        vec![Tx{
            amount: 50,
            sender: [1; 20],
            receiver: [3; 20],
            status: TxStatus::Confirmed(2),
        }],
    ).unwrap());

    println!("total blockchain volume = {}", Blockchain::total_volume(&blockchain));
}

struct Blockchain {
    blocks: Vec<Block>
}

impl Blockchain {
    fn add_new(&mut self, block: Block) {
        
        self.blocks.push(block);
        
    }

    fn find_txs(&self, amount: u64) -> Vec<&Tx> {
        self.blocks.iter().flat_map(|block| block.txs.iter()).filter(|tx| tx.amount == amount).collect()
    }

    fn total_volume(&self) -> u64 {
        self.blocks.iter().flat_map(|block| block.txs.iter()).map(|tx| tx.amount).sum()
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
    previous_hash: Option<[u8; 32]>,
    merkle_root: [u8; 32],
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