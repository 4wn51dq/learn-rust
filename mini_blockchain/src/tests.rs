#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::{Block, Header};
    use crate::transaction::{Tx, TxStatus};
    use crate::chain::Blockchain;
    use crate::hasher::SHA256;

    #[test]
    fn test_addnew_rejects_block_w_empty_tx() {
        let header = Header {
            previous_hash: Some([67; 32]),
        merkle_root: [8; 32],
        nonce: 291356,
        timestamp: 0987654321,
        };
        let result = Block::new(
            header,
            vec![]
        );
        assert!(result.is_err());
    }

    #[test]
    fn valid_tx_in_block_new_returns_ok() {
        let header = Header {
            previous_hash: Some([67; 32]),
        merkle_root: [8; 32],
        nonce: 291356,
        timestamp: 0987654321,
        };
        let txs: Vec<Tx> = vec![
            Tx{
                amount: 100,
                sender: [1; 20],
                receiver: [2;20],
                status: TxStatus::Pending,
            }
        ];
        let result = Block::new(
            header,
            txs,
        );
        assert!(result.is_ok());
    }

    #[test] // integ
    fn addnew_gives_error_on_wrong_prevhash_and_only_genenis_is_allowed_no_prevhash() {
        let mut chain = Blockchain{blocks: vec![]};
        let genesis = Block {
            header: Header {
                previous_hash: None,
                merkle_root: [0; 32],
                nonce: 1,
                timestamp: 0,
            },
            txs: vec![Tx{
                amount: 1,
                sender: [1; 20],
                receiver: [2; 20],
                status: TxStatus::Confirmed(1),
            }]
        };
        let header = Header {
            previous_hash: Some([67; 32]),
            merkle_root: [8; 32],
            nonce: 291356,
            timestamp: 0987654321,
        };
        let txs: Vec<Tx> = vec![
            Tx{
                amount: 100,
                sender: [1; 20],
                receiver: [2;20],
                status: TxStatus::Pending,
            }
        ];
        let block1 = Block::new(
            header,
            txs,
        ).unwrap();
        let block1_clone = block1.clone();
        let result1 = Blockchain::add_new(&mut chain, block1, &SHA256{});
        assert!(result1.is_err());

        chain.blocks.push(genesis);
        let result2 = Blockchain::add_new(&mut chain, block1_clone, &SHA256{});
        assert!(result2.is_err());

    }
}