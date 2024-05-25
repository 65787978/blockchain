use chrono::{DateTime, Utc};

struct PowSolution {
    pk: String,
    w: String,
    n: String,
    d: String,
}
#[derive(Debug)]
pub struct Block {
    pub height: u32,
    timestamp: DateTime<Utc>,
    pub hash: &'static str,
    previous_block_hash: &'static str,
    version: &'static str,
}
#[derive(Debug)]
pub struct BlockChain {
    pub chain: Vec<Block>,
}
impl BlockChain {
    pub fn genesis_block() -> Self {
        BlockChain {
            chain: vec![Block {
                height: 0,
                timestamp: Utc::now(),
                hash: "0",
                previous_block_hash: "0",
                version: "0",
            }],
        }
    }

    pub fn add_block(&mut self) {
        let old_block = self.chain.last().unwrap();
        /* Add a new block to the blockchain */
        self.chain.push(Block {
            height: old_block.height + 1,
            timestamp: Utc::now(),
            hash: "-",
            previous_block_hash: old_block.hash,
            version: "0",
        })
    }
}
