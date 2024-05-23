use chrono::{DateTime, Local, Utc};

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
impl Block {
    pub fn new(
        height: u32,
        hash: &'static str,
        previous_block_hash: &'static str,
        version: &'static str,
    ) -> Block {
        Block {
            height: height,
            timestamp: Local::now().with_timezone(&Utc),
            hash: hash,
            previous_block_hash: previous_block_hash,
            version: version,
        }
    }
}
