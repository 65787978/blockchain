use crate::block::*;
use json::*;
use std::fs::File;
use std::io::Write;
use std::{thread::sleep, time::Duration};
mod block;
fn main() {
    println!("Hello, world!");

    /* Genesis Block info */
    let mut chain: Vec<Block> = vec![Block::new(0, "0", "0", "0")];

    let mut db = File::create("DB").expect("Creation of database failed!");

    loop {
        println!("\n{:#?},", chain[chain.len() - 1]);
        write!(db, "{:#?},\n", chain[chain.len() - 1]).expect("Saving failed");

        chain.push(Block::new(
            chain[chain.len() - 1].height + 1,
            "hash",
            chain[chain.len() - 1].hash,
            "0",
        ));

        /* If the chain vec holds more than 5 blocks, remove the first block */
        if chain.len() > 4 {
            chain.remove(0);
        }

        sleep(Duration::from_secs(1));
    }
}
