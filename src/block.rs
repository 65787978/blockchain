use std::vec;

use blake2::{Blake2b512, Digest};
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Block {
    pub data: Vec<String>,
    pub previous_block_hash: String,
    pub nonce: u64,
    pub hash: String,

    pub timestamp: DateTime<Utc>,
    pub height: u64,
}
impl Block {
    fn genesis_block(data: &Vec<String>) -> Block {
        Block {
            data: data.clone(),
            previous_block_hash: hash_data(vec!["0".to_string()]),
            nonce: 01,
            hash: hash_data(data.clone()),
            timestamp: Utc::now(),
            height: 0,
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
        let mut new_block = Block {
            data: data.clone(),
            previous_block_hash: old_block.hash.to_owned(),
            nonce: 01,
            hash: hash_data(data), //the data needs to be hashed only after the block is mined
            timestamp: Utc::now(),
            height: old_block.height + 1,
        };
        self.chain.push(new_block);
    }
}

fn hash_data(data: Vec<String>) -> String {
    let mut hasher = Blake2b512::new();
    /*Separating each transaction with ',' and joining them before hashing*/
    let string_data = data.join(",");

    hasher.update(string_data.as_bytes());
    hex::encode(hasher.finalize())
}
