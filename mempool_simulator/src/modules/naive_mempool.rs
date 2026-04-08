use std::{collections::BTreeSet, sync::{Arc, atomic::AtomicU64}};

use dashmap::DashMap;
use parking_lot::{RwLock, RwLockWriteGuard};

use crate::modules::{reverse_gas::ReverseGas, transaction_model::Transaction, types::{GasPrice, TxHash}};


const MAX_SIZE: usize = 10_000; // Example max size for the mempool


pub struct NaiveMempool {
    pub txs_store: DashMap<TxHash, Transaction>,
    pub txs_by_gas_price: Arc<RwLock<BTreeSet<(ReverseGas , TxHash)>>>, // We will store gas prices in reverse order for easy retrieval of highest fee txs
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
        // Check if we have space in the mempool

        let mut write_lock = self.txs_by_gas_price.write();

        if self.txs_store.len() >= MAX_SIZE {
         if write_lock.iter().next_back().map(|obj| obj.0.0) < Some(tx.gas_price) {
            self.remove_worst_lock(&mut write_lock); // Evict the worst transaction to make space
         } else {
            self.total_rejected.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            return; // Reject the new transaction if it's not better than the worst one
         }
        }

        // Add to store
        self.txs_store.insert(tx.hash , tx.clone());
        
        // Add to price tree
        // let mut set = self.txs_by_gas_price.write();
        write_lock.insert((ReverseGas(tx.gas_price), tx.hash));
        drop(write_lock); // Release lock immediately after modification

        self.total_added.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn get_all(&self) -> Vec<Transaction> {
       let txs: Vec<Transaction> =  self.txs_store.iter().map(|r| r.value().clone()).collect();
       txs
    }

      pub fn get_price_hash_tree(&self) -> Vec<ReverseGas> {
       self.txs_by_gas_price.read().iter().map(|obj| &obj.0).cloned().collect::<Vec<ReverseGas>>()
        
    }

      pub fn get_price_tree(&self) -> Vec<GasPrice> {
        self.txs_by_gas_price.read().iter().map(|obj|&obj.0.0).cloned().collect::<Vec<GasPrice>>()
    }

    pub fn get_by_hash(&self, hash: &TxHash) -> Option<Transaction> {
     
     let res = match self.txs_store.get(hash) {
        Some(r) => Some(r.value().clone()),
        None => None
     };

     res
    }

    pub fn get_best(&self, n: usize) -> Vec<Transaction> {
        // Get Data and release lock fast . 
        let hashes : Vec<(ReverseGas , TxHash)> =    self.txs_by_gas_price
     .read()
     .iter()
     .take(n).cloned().collect(); 


     // Do the dashmap lookup after the lock has been released . 
     hashes.into_iter().map(|obj| self.get_by_hash(&obj.1).unwrap())
     .collect::<Vec<Transaction>>()

     

    }

    pub fn get_best_price(&self) -> Option<GasPrice> {
        self.txs_by_gas_price.read().iter().next().map(|obj| obj.0.0)
    }


      pub fn remove_worst(&self) -> Option<Transaction> {

        // Avoid doing RwLock read() and write() together inside one thread , because the thread locks between both operation and goes to another thread before coming back . 
        // In this situation bellow , we do not want that , cause it'll lead to data inconsistency between the threads .  

        // FIX 2: Acquire Write Lock immediately for atomic check-and-remove
        let mut set = self.txs_by_gas_price.write();
        
        // Find the worst (highest gas price in reverse sort? No, usually worst is lowest fee)
        // Assuming ReverseGas sorts High->Low, iter().next_back() is the lowest fee.
        if let Some((price_key, hash)) = set.iter().next_back().cloned() {
            set.remove(&(price_key, hash.clone()));
            drop(set); // Release lock immediately after modification
            let tx = self.txs_store.remove(&hash);
            self.total_evicted.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            Some(tx.unwrap().1) // or pan
        } else {
            None
        }
    }


       pub fn remove_worst_lock(&self , write_lock: &mut RwLockWriteGuard<'_, BTreeSet<(ReverseGas, TxHash)>>) -> Option<Transaction> {

        // Avoid doing RwLock read() and write() together inside one thread , because the thread locks between both operation and goes to another thread before coming back . 
        // In this situation bellow , we do not want that , cause it'll lead to data inconsistency between the threads .  

        // FIX 2: Acquire Write Lock immediately for atomic check-and-remove
        // let mut set = self.txs_by_gas_price.write();
        
        // Find the worst (highest gas price in reverse sort? No, usually worst is lowest fee)
        // Assuming ReverseGas sorts High->Low, iter().next_back() is the lowest fee.
        if let Some((price_key, hash)) = write_lock.iter().next_back().cloned() {
            write_lock.remove(&(price_key, hash.clone()));
            // drop(write_lock); // Release lock immediately after modification
            let tx = self.txs_store.remove(&hash);
            self.total_evicted.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            Some(tx.unwrap().1) // or pan
        } else {
            None
        }
    }


    // pub fn mine_block(&self, max_gas: u64) -> Vec<Transaction> {
       
    //         let mut gas_used = 0;    
    //         let read_lock = self.txs_by_gas_price.read();

    //         // Get Data and release lock fast . 
    //         let hashes : Vec<(ReverseGas , TxHash)> =    read_lock
    //         .iter()
    //         .take_while(|obj| {
    //         if let Some(tx) = self.get_by_hash(&obj.1){
    //         if gas_used + tx.gas_limit <= max_gas {
                
    //             gas_used += tx.gas_limit;
    //             true
    //         } else {
    //             false
    //         }
    //         }else {
    //             false 
    //         }

    //     })
    //     .cloned().collect();
    //     drop(read_lock);

    //     let mut write_lock = self.txs_by_gas_price.write();
    //     let mut mined_txs = Vec::new();
    //     for (price_key, hash) in hashes {
    //         // Remove from price tree
    //         write_lock.remove(&(price_key, hash.clone()));
    //         // Remove from store
    //         if let Some(tx) = self.txs_store.remove(&hash) {
    //             mined_txs.push(tx.1);
    //             self.total_evicted.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    //         }
    //     }
    // mined_txs
    // }



    // ... other methods ...

    pub fn mine_block(&self, max_gas: u64) -> Vec<Transaction> {
        // 1. ACQUIRE READ LOCK
        let read_lock = self.txs_by_gas_price.read();
        
        let mut gas_used = 0;
        let mut candidates: Vec<(ReverseGas, TxHash)> = Vec::new();

        // 2. ITERATE ALL (Don't use take_while)
        // We iterate through the sorted set (Best -> Worst)
        for (rev_gas, hash) in read_lock.iter() {
            // We need the gas limit. We access DashMap while holding the ReadLock.
            // This is safe because DashMap allows concurrent read/write, 
            // and the BTreeSet ReadLock allows concurrent reads.
            if let Some(tx) = self.txs_store.get(hash) {
                if gas_used + tx.gas_limit <= max_gas {
                    gas_used += tx.gas_limit;
                    candidates.push((rev_gas.clone(), hash.clone()));
                }
                // Optimization: If block is nearly full, we could break early, 
                // but we must continue checking smaller txs.
            }
        }
        
        // 3. RELEASE READ LOCK
        drop(read_lock);

        // 4. ACQUIRE WRITE LOCK FOR REMOVAL
        let mut write_lock = self.txs_by_gas_price.write();
        let mut mined_txs = Vec::new();

        for (price_key, hash) in candidates {
            // Remove from priority index
            if let true = write_lock.remove(&(price_key, hash.clone())) {
                // Remove from storage
                if let Some((_, tx)) = self.txs_store.remove(&hash) {
                    mined_txs.push(tx);
                    self.total_mined.fetch_add(1, std::sync::atomic::Ordering::Relaxed); // FIX: Track mined, not evicted
                }
            }
        }

        mined_txs
    }



}