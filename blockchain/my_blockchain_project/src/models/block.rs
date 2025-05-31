use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Block {
    index: u32,
    timestamp: u64,
    data: String,
    previous_hash: String,
    hash: String,
    nonce: u32,
}


impl Block {
    fn new(index: u32, data: String, previous_hash: String) -> Block {
        let timestamp = get_current_timestamp();
        let nonce = 0; // Initial nonce
        let hash = calculate_hash(index, &previous_hash, timestamp, &data, nonce);
        Block { index, timestamp, data, previous_hash, hash, nonce }
    }
}