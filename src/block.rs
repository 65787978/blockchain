use std::vec;

use blake2::{Blake2b512, Digest};
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Block {
    pub height: u64,
    pub timestamp: DateTime<Utc>,
    pub hash: String,
    pub previous_block_hash: String,
    pub nonce: u64,
    pub data: Vec<String>,
    pub version: String,
}
impl Block {
    fn genesis_block(data: &Vec<String>) -> Block {
        Block {
            height: 0,
            timestamp: Utc::now(),
            hash: hash_data(data.clone()),
            previous_block_hash: hash_data(vec!["0".to_string()]),
            nonce: 01,
            data: data.clone(),
            version: "0".to_string(),
        }
    }
}
#[derive(Debug)]
pub struct BlockChain {
    pub chain: Vec<Block>,
}
impl BlockChain {
    pub fn new(data: &Vec<String>) -> Self {
        BlockChain {
            chain: vec![Block::genesis_block(data)],
        }
    }

    pub fn add_block(&mut self, data: Vec<String>) {
        let old_block = self.chain.last().unwrap();
        /* Add a new block to the blockchain */
        self.chain.push(Block {
            height: old_block.height + 1,
            timestamp: Utc::now(),
            hash: hash_data(data.clone()), //the data needs to be hashed only after the block is mined
            previous_block_hash: old_block.hash.to_owned(),
            nonce: 01,
            data: data,
            version: "0".to_string(),
        })
    }
}

fn hash_data(data: Vec<String>) -> String {
    let mut hasher = Blake2b512::new();
    let string_data = data.join("");

    hasher.update(string_data.as_bytes());
    hex::encode(hasher.finalize())
}
