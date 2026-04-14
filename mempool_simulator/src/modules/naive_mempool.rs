use std::{ collections::BTreeSet, sync::{ Arc, atomic::{ AtomicU64, Ordering } } };

use dashmap::DashMap;
use parking_lot::{ RwLock, RwLockReadGuard, RwLockWriteGuard };

use crate::modules::{
    reverse_gas::ReverseGas,
    transaction_model::Transaction,
    types::{ GasPrice, TxHash },
};

const MAX_SIZE: usize = 10_000; // Example max size for the mempool

pub struct NaiveMempool {
    pub txs_store: DashMap<TxHash, Transaction>,
    pub txs_by_gas_price: Arc<RwLock<BTreeSet<(ReverseGas, TxHash)>>>, // We will store gas prices in reverse order for easy retrieval of highest fee txs
    pub total_added: AtomicU64, // To track total transactions ever added for unique hash generation
    pub total_evicted: AtomicU64, // To track total transactions ever added for unique hash generation
    pub total_rejected: AtomicU64, // To track total transactions ever added for unique hash generation
    pub total_mined: AtomicU64, // To track total transactions ever added for unique hash generation
}

impl NaiveMempool {
    pub fn new() -> Self {
        NaiveMempool {
            txs_store: DashMap::new(),
            txs_by_gas_price: Arc::new(RwLock::new(BTreeSet::new())),
            total_added: AtomicU64::new(0),
            total_evicted: AtomicU64::new(0),
            total_rejected: AtomicU64::new(0),
            total_mined: AtomicU64::new(0),
        }
    }

    pub fn add(&self, tx: &Transaction) {
        if self.txs_store.len() >= MAX_SIZE {
            let read_lock = self.txs_by_gas_price.read();

            if
                read_lock
                    .iter()
                    .next_back()
                    .map(|obj| obj.0.0) < Some(tx.gas_price)
            {
                let reference_gas = read_lock.iter().next_back().cloned();
                   drop(read_lock);

                if let Some(ref_gas) = reference_gas {
                 

                    let mut write_lock = self.txs_by_gas_price.write();
                    let removed = self.remove_worst_lock(&mut write_lock, &ref_gas);
                    drop(write_lock);

                    if removed {
                        self.txs_store.remove(&ref_gas.1);
                        self.total_evicted.fetch_add(1, Ordering::Relaxed);
                        self.txs_store.insert(tx.hash, tx.clone());
                        let mut write_lock = self.txs_by_gas_price.write();
                        write_lock.insert((ReverseGas(tx.gas_price), tx.hash));
                        self.total_added.fetch_add(1, Ordering::Relaxed); // ✅ only here
                    } else {
                        self.total_rejected.fetch_add(1, Ordering::Relaxed);
                    }
                }
                // no fetch_add here
            } else {
                self.total_rejected.fetch_add(1, Ordering::Relaxed);
            }
            // no fetch_add here
        } else {
            let mut write_lock = self.txs_by_gas_price.write();
            self.txs_store.insert(tx.hash, tx.clone());
            write_lock.insert((ReverseGas(tx.gas_price), tx.hash));
            self.total_added.fetch_add(1, Ordering::Relaxed); // ✅ only here
        }
        // ← nothing here
    }

    pub fn get_all(&self) -> Vec<Transaction> {
        let txs: Vec<Transaction> = self.txs_store
            .iter()
            .map(|r| r.value().clone())
            .collect();
        txs
    }

    pub fn get_price_hash_tree(&self) -> Vec<ReverseGas> {
        self.txs_by_gas_price
            .read()
            .iter()
            .map(|obj| &obj.0)
            .cloned()
            .collect::<Vec<ReverseGas>>()
    }

    pub fn get_price_tree(&self) -> Vec<GasPrice> {
        self.txs_by_gas_price
            .read()
            .iter()
            .map(|obj| &obj.0.0)
            .cloned()
            .collect::<Vec<GasPrice>>()
    }

  // After
pub fn get_by_hash(&self, hash: &TxHash) -> Option<Transaction> {
    self.txs_store.get(hash).map(|r| r.value().clone())
}

    pub fn get_best(&self, n: usize) -> Vec<Transaction> {
        // Get Data and release lock fast .
        let hashes: Vec<(ReverseGas, TxHash)> = self.txs_by_gas_price
            .read()
            .iter()
            .take(n)
            .cloned()
            .collect();

        // Do the dashmap lookup after the lock has been released .
        hashes
            .into_iter()
            .filter_map(|obj| self.get_by_hash(&obj.1))
            .collect::<Vec<Transaction>>()
    }

    pub fn get_best_price(&self) -> Option<GasPrice> {
        self.txs_by_gas_price
            .read()
            .iter()
            .next()
            .map(|obj| obj.0.0)
    }



    pub fn remove_worst_lock(
        &self,
        write_lock: &mut RwLockWriteGuard<'_, BTreeSet<(ReverseGas, TxHash)>>,
        reverse_gas_: &(ReverseGas, TxHash)
    ) -> bool {
        write_lock.remove(reverse_gas_) // returns true if it was present
    }

  

    pub fn mine_block(&self, max_gas: u64) -> Vec<Transaction> {
        let mut gas_used = 0;

        // Get Data and release lock fast .
        let hashes: Vec<(ReverseGas, TxHash)> = {
            let read_lock = self.txs_by_gas_price.read();
            read_lock
                .iter()
                .filter_map(|obj| {
                    if let Some(tx) = self.get_by_hash(&obj.1) {
                        if gas_used + tx.gas_limit <= max_gas {
                            gas_used += tx.gas_limit;
                            Some(obj)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .cloned()
                .collect()
        }; // read lock is dropped here .

        {
            let mut write_lock = self.txs_by_gas_price.write();
            for hash in hashes.iter() {
                // Remove from price tree
                write_lock.remove(hash);
                // Remove from store
                // if let Some(tx) = self.txs_store.remove(&hash) {
                //     mined_txs.push(tx.1);
                //     self.total_mined.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                // }
            }
        }

        //     // Phase 3: remove from DashMap with no locks held
        hashes
            .into_iter()
            .filter_map(|(_, hash)| {
                self.txs_store.remove(&hash).map(|(_, tx)| {
                    self.total_mined.fetch_add(1, Ordering::Relaxed);
                    tx
                })
            })
            .collect()

        // write lock is droped here .
    }
}
