use dashmap::DashMap;
use parking_lot::RwLock;
use rand::{Rng, RngExt};
use rayon::prelude::*;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use tokio::time::sleep;

// ============================================================================
// Constants & Configuration
// ============================================================================

const MAX_MEMPOOL_SIZE: usize = 100_000;
const MAX_ACCOUNT_TXS: usize = 16;
const TARGET_TPS: usize = 15_000;
const BLOCK_GAS_LIMIT: u64 = 30_000_000;

// ============================================================================
// Data Structures
// ============================================================================

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct TxHash([u8; 32]);

impl TxHash {
    fn to_hex(&self) -> String {
        hex::encode(&self.0[..4])
    }
}

#[derive(Debug, Clone)]
pub struct Transaction {
    hash: TxHash,
    from: String,
    gas_price: u64,
    gas_limit: u64,
    size: u64,
    nonce: u64,
    timestamp: Instant,
}

impl Transaction {
    fn priority_key(&self) -> (ReverseGas, TxHash) {
        (ReverseGas(self.gas_price), self.hash.clone())
    }
}

// Wrapper to reverse gas price ordering for BTreeSet
#[derive(Debug, Clone, Ord)]
struct ReverseGas(u64);
impl Eq for ReverseGas {}
impl PartialEq for ReverseGas {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl PartialOrd for ReverseGas {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0).map(|o| o.reverse())
    }
}

// ============================================================================
// Mempool Core
// ============================================================================

pub struct Mempool {
    transactions: DashMap<TxHash, Transaction>,
    account_txs: DashMap<String, BTreeSet<TxHash>>,
    priority_index: RwLock<BTreeSet<(ReverseGas, TxHash)>>,

    // Metrics
    total_added: AtomicU64,
    total_evicted: AtomicU64,
    total_rejected: AtomicU64,
    total_mined: AtomicU64,
    current_size: AtomicU64,
}

impl Mempool {
    pub fn new() -> Self {
        Self {
            transactions: DashMap::new(),
            account_txs: DashMap::new(),
            priority_index: RwLock::new(BTreeSet::new()),
            total_added: AtomicU64::new(0),
            total_evicted: AtomicU64::new(0),
            total_rejected: AtomicU64::new(0),
            total_mined: AtomicU64::new(0),
            current_size: AtomicU64::new(0),
        }
    }

    pub fn add_tx(&self, tx: Transaction) -> bool {
        // 1. Check account limit
        let mut account_entry = self.account_txs
            .entry(tx.from.clone())
            .or_insert_with(BTreeSet::new);

        if account_entry.len() >= MAX_ACCOUNT_TXS {
            self.total_rejected.fetch_add(1, Ordering::Relaxed);
            return false;
        }

        // 2. Check capacity & Evict if necessary
        if self.transactions.len() >= MAX_MEMPOOL_SIZE {
            if !self.evict_lowest() {
                self.total_rejected.fetch_add(1, Ordering::Relaxed);
                return false;
            }
        }

        // 3. Insert
        let hash = tx.hash.clone();
        let size = tx.size;

        account_entry.insert(hash.clone());
        drop(account_entry); // Release DashMap shard lock

        self.transactions.insert(hash.clone(), tx.clone());
        self.priority_index.write().insert(tx.priority_key());

        self.total_added.fetch_add(1, Ordering::Relaxed);
        self.current_size.fetch_add(size, Ordering::Relaxed);
        true
    }

    fn evict_lowest(&self) -> bool {
        let mut priority = self.priority_index.write();

        if let Some(first) = priority.iter().next().cloned() {
            priority.remove(&first);
            drop(priority); 

            let (_, hash) = first;

            if let Some((_, tx)) = self.transactions.remove(&hash) {
                self.current_size.fetch_sub(tx.size, Ordering::Relaxed);
                if let Some(mut acc_set) = self.account_txs.get_mut(&tx.from) {
                    acc_set.remove(&hash);
                }
                self.total_evicted.fetch_add(1, Ordering::Relaxed);
                return true;
            }
        }
        false
    }

    pub fn mine_block(&self, max_gas: u64) -> Vec<Transaction> {
        let mut block = Vec::new();
        let mut gas_used = 0;
        let mut to_remove = Vec::new();

        let priority = self.priority_index.read();
        for (rev_gas, hash) in priority.iter().rev() {
            if gas_used + 21_000 > max_gas { break; }

            if let Some(tx) = self.transactions.get(hash) {
                gas_used += 21_000;
                block.push(tx.clone());
                to_remove.push((rev_gas.clone(), hash.clone()));
            }
        }
        drop(priority);

        let mut priority = self.priority_index.write();
        for key in to_remove {
            priority.remove(&key);
            self.transactions.remove(&key.1);
        }

        self.total_mined.fetch_add(block.len() as u64, Ordering::Relaxed);
        block
    }

    pub fn stats(&self) -> (usize, u64, u64, u64, u64, u64) {
        (
            self.transactions.len(),
            self.total_added.load(Ordering::Relaxed),
            self.total_evicted.load(Ordering::Relaxed),
            self.total_rejected.load(Ordering::Relaxed),
            self.total_mined.load(Ordering::Relaxed),
            self.current_size.load(Ordering::Relaxed),
        )
    }
}

// ============================================================================
// Simulation Engine
// ============================================================================

pub struct Simulator {
    mempool: Mempool,
    tps_counter: AtomicU64,
    target_tps: usize,
}

// impl Clone for Simulator {
//     fn clone(&self) -> Self {
//         Self { 
//             mempool: self.mempool.clone(), 
//             tps_counter: AtomicU64::new(self.tps_counter.load(Ordering::Relaxed)), 
//             target_tps: self.target_tps.clone() 
//         }
//     }
// }



impl Simulator {
    pub fn new(target_tps: usize) -> Self {
        Self {
            mempool: Mempool::new(),
            tps_counter: AtomicU64::new(0),
            target_tps,
        }
    }

    // fn gen_tx(i: usize) -> Transaction {
    //     let mut rng = rand::thread_rng();
    //     let mut hasher = Sha256::new();
    //     let random_seed: u64 = rng.gen();
    //     hasher.update(format!("tx-{}-{}", i, random_seed));
    //     let result = hasher.finalize();
    //     let mut hash_bytes = [0u8; 32];
    //     hash_bytes.copy_from_slice(&result);

    //     Transaction {
    //         hash: TxHash(hash_bytes),
    //         from: format!("0x{:040x}", rng.gen::<u32>()),
    //         to: format!("0x{:040x}", rng.gen::<u32>()),
    //         gas_price: rng.gen_range(20..200),
    //         gas_limit: 21_000,
    //         size: rng.gen_range(200..1000),
    //         nonce: rng.gen(),
    //         timestamp: Instant::now(),
    //     }
    // }

        fn gen_tx(i: usize) -> Transaction {
        // // let mut rng = rand::thread_rng();
        // let mut hasher = Sha256::new();
        // hasher.update(format!("tx-{}-{}", i, rng.gen::<u64>()));
        // let result = hasher.finalize();
        // let mut hash_bytes = [0u8; 32];
        // hash_bytes.copy_from_slice(&result);

        let mut rng = rand::rng();
        let mut hasher = Sha256::new();
        let random_seed: u64 = rng.next_u64();
        // hasher.update(random_seed.to_le_bytes());
        hasher.update(format!("tx-{}-{}", i, random_seed));
        let result = hasher.finalize();
        let mut hash_bytes = [0u8; 32];
        hash_bytes.copy_from_slice(&result);

        Transaction {
            hash: TxHash(hash_bytes),
            from: format!("0x{:040x}", rng.next_u32()),
            // to: format!("0x{:040x}", rng.next_u32()),
            gas_price: rng.random_range(20..200),
            gas_limit: 21_000,
            size: rng.random_range(200..1000),
            nonce: rng.random(),
            timestamp: Instant::now(),
        }
    }

    pub async fn run(&self) {
        let mempool_producer = self.mempool.clone();
        let tps_counter =   AtomicU64::new(self.tps_counter.load(Ordering::Relaxed)); 
        let target = self.target_tps;

        // Task 1: Producer
        tokio::spawn(async move {
            let batch_size = 1000;
            loop {
                let txs: Vec<Transaction> = (0..batch_size)
                    .into_par_iter()
                    .map(|i| Simulator::gen_tx(i))
                    .collect();

                for tx in txs {
                    mempool_producer.add_tx(tx);
                    tps_counter.fetch_add(1, Ordering::Relaxed);
                }

                let sleep_ms = (((batch_size as f64) / (target as f64)) * 1000.0) as u64;
                if sleep_ms > 0 {
                    sleep(Duration::from_millis(sleep_ms)).await;
                }
            }
        });

        // Task 2: Miner
        let mempool_miner = self.mempool.clone();
        tokio::spawn(async move {
            loop {
                sleep(Duration::from_secs(2)).await;
                let mined = mempool_miner.mine_block(BLOCK_GAS_LIMIT);
                if !mined.is_empty() {
                    println!("[MINER] ⛏  Mined {} transactions", mined.len());
                }
            }
        });

        // Task 3: Metrics
        let mempool_metrics = self.mempool.clone();
        let tps_metrics = self.tps_counter.clone();
        tokio::spawn(async move {
            let mut last_count = 0u64;
            loop {
                sleep(Duration::from_secs(1)).await;
                
                let current_count = tps_metrics.load(Ordering::Relaxed);
                let tps = current_count - last_count;
                last_count = current_count;

                let (tx_count, added, evicted, rejected, mined, size) = mempool_metrics.stats();
                let size_mb = size as f64 / 1_000_000.0;

                println!(
                    "[METRICS] TPS: {:>6} | Pool: {:>6} | Added: {:>7} | Mined: {:>6} | Evicted: {:>6} | Rejected: {:>7} | Size: {:.2} MB",
                    tps, tx_count, added, mined, evicted, rejected, size_mb
                );
            }
        });
    }
}

// Implement Clone for Mempool (DashMap is Arc-based)
impl Clone for Mempool {
    fn clone(&self) -> Self {
        Self {
            transactions: self.transactions.clone(),
            account_txs: self.account_txs.clone(),
            priority_index: self.priority_index.clone(),
            total_added: AtomicU64::new(self.total_added.load(Ordering::Relaxed)),
            total_evicted: AtomicU64::new(self.total_evicted.load(Ordering::Relaxed)),
            total_rejected: AtomicU64::new(self.total_rejected.load(Ordering::Relaxed)),
            total_mined: AtomicU64::new(self.total_mined.load(Ordering::Relaxed)),
            current_size: AtomicU64::new(self.current_size.load(Ordering::Relaxed)),
        }
    }
}

// ============================================================================
// Main Entry
// ============================================================================

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let target_tps = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(10_000);

    println!("Starting Mempool Simulator targeting {} tx/s...", target_tps);
    println!("Press Ctrl+C to stop.\n");

    let simulator = Simulator::new(target_tps);
    simulator.run().await;

    // Keep main alive
    tokio::signal::ctrl_c().await.ok();
    println!("\nSimulation stopped.");
}