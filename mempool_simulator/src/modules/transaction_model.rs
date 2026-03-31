use rand::{Rng, RngExt, random};
use sha2::{Digest, Sha256};

use crate::modules::types::TxHash;


#[derive(Clone, Debug)]
pub struct Transaction {
    pub hash: TxHash,   // Unique ID
    pub from: String,
    pub to: String,
    pub size: u64, 
    pub gas_price: u64,
    pub gas_limit: u64,
    pub nonce: u64
}

impl Transaction {
    fn new(hash: TxHash, from: String, to: String, size: u64, gas_price: u64, gas_limit: u64, nonce: u64) -> Self {
        Transaction {
            hash,
            from,
            to,
            size,
            gas_price,
            gas_limit,
            nonce
        }
    }

}



/// Helper function to generate a deterministic but realistic Transaction
pub fn generate_tx(nonce: u64) -> Transaction {
    let mut rng = rand::rng();

    // 1. Create the Hash (Unique ID)
    // We use a SHA256 hasher.
    let mut hasher = Sha256::new();
    
    // We feed the nonce into the hasher. 
    // To ensure uniqueness even with the same nonce across runs, we mix in random bytes.
    // In a real blockchain, you would hash (Sender + Nonce + Data).
    let random_seed: u64 = rng.next_u64();
    hasher.update(nonce.to_le_bytes());
    hasher.update(random_seed.to_le_bytes());
    
    // Finalize the hash
    let result = hasher.finalize();
    
    // Convert the result (GenericArray) into a fixed Hash array
    let mut hash_bytes = [0u8; 32];
    hash_bytes.copy_from_slice(&result);

    // 2. Generate Dummy Data for other fields
    Transaction {
        hash: hash_bytes,
        from: format!("0x{:040x}", rng.next_u32()), // Random 40-char hex string
        to: format!("0x{:040x}", rng.next_u32()),
        gas_price: rng.random_range(10..1000),           // Random fee
        gas_limit: 21_000,                            // Standard transfer limit
        size: rng.random_range(100..500),                // Random size
        nonce,
    }
}