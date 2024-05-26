use std::vec;

use blake2::{Blake2b512, Digest};
use chrono::{DateTime, Utc};
struct PowSolution {
    pk: String,
    w: String,
    n: String,
    d: String,
}
#[derive(Debug)]
pub struct Block {
    pub height: u64,
    pub timestamp: DateTime<Utc>,
    pub hash: String,
    pub previous_block_hash: String,
    pub nonce: u64,
    pub data: String,
    pub version: String,
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
                hash: hash_it("GenesisBlockNakamoto".to_string()),
                previous_block_hash: hash_it("0".to_string()),
                nonce: 01,
                data: "Nothing yet".to_string(),
                version: "0".to_string(),
            }],
        }
    }

    pub fn add_block(&mut self) {
        let old_block = self.chain.last().unwrap();
        /* Add a new block to the blockchain */
        self.chain.push(Block {
            height: old_block.height + 1,
            timestamp: Utc::now(),
            hash: hash_it(self.chain.last().unwrap().timestamp.to_string()),
            previous_block_hash: old_block.hash.to_owned(),
            nonce: 01,
            data: "Nothing yet".to_string(),
            version: "0".to_string(),
        })
    }
}

fn hash_it(data: String) -> String {
    let mut hasher = Blake2b512::new();
    hasher.update(data);
    hex::encode(hasher.finalize().to_vec())
}
