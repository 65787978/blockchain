use crate::block::*;
use json::*;
use std::fs::File;
use std::io::Write;
use std::{thread::sleep, time::Duration};
mod block;
fn main() {
    /* Genesis Block info */
    let mut block_chain = BlockChain::new(&vec!["GenesisBlock".to_string()]);

    // let mut db = File::create("DB").expect("Creation of database failed!");

    loop {
        sleep(Duration::from_secs(1));
        println!("\n{:#?},", block_chain.chain[block_chain.chain.len() - 1]);
        // write!(db, "{:#?},\n", chain[chain.len() - 1]).expect("Saving failed");

        block_chain.add_block(vec!["Transactions".to_string()]);

        /* If the chain vec holds more than 5 blocks, remove the first block */
        // if chain.len() > 4 {
        //     chain.remove(0);
        // }
    }
}
