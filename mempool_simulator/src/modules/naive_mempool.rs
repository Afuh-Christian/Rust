use std::{collections::{BTreeSet}, sync::Arc};

use dashmap::DashMap;
use parking_lot::RwLock;

use crate::modules::{reverse_gas::ReverseGas, transaction_model::Transaction, types::{GasPrice, TxHash}};

pub struct NaiveMempool {
    pub txs_store: DashMap<TxHash, Transaction>,
    pub txs_by_gas_price: Arc<RwLock<BTreeSet<(ReverseGas , TxHash)>>>, // We will store gas prices in reverse order for easy retrieval of highest fee txs
}

impl NaiveMempool {
    pub fn new() -> Self {
        NaiveMempool {
            txs_store: DashMap::new(),
            txs_by_gas_price: Arc::new(RwLock::new(BTreeSet::new())),
        }
    }

    pub fn add(&self, tx: &Transaction) {
        self.txs_store.insert(tx.hash , tx.clone());
        self.txs_by_gas_price.write().insert((ReverseGas(tx.gas_price), tx.hash));
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
            Some(tx.unwrap().1) // or pan
        } else {
            None
        }
    }


}






// use std::{cmp::Reverse, collections::{BTreeSet, HashMap}, hash::Hash};

// use crate::modules::{reverse_gas::ReverseGas, transaction_model::Transaction, types::{GasPrice, TxHash}};

// pub struct NaiveMempool {
//     pub txs_store: HashMap<TxHash, Transaction>,
//     pub txs_by_gas_price: BTreeSet<(ReverseGas , TxHash)>, // We will store gas prices in reverse order for easy retrieval of highest fee txs
// }

// impl NaiveMempool {
//     pub fn new() -> Self {
//         NaiveMempool {
//             txs_store: HashMap::new(),
//             txs_by_gas_price: BTreeSet::new(),
//         }
//     }

//     pub fn add(&mut self, tx: &Transaction) {
//         self.txs_store.insert(tx.hash , tx.clone());
//         self.txs_by_gas_price.insert((ReverseGas(tx.gas_price), tx.hash));
//     }

//     pub fn get_all(&self) -> Vec<Transaction> {
//         self.txs_store.values().cloned().collect()
//     }

//       pub fn get_price_hash_tree(&self) -> Vec<&ReverseGas> {
//         self.txs_by_gas_price.iter().map(|obj| &obj.0).collect::<Vec<&ReverseGas>>()
//     }

//       pub fn get_price_tree(&self) -> Vec<&GasPrice> {
//         self.txs_by_gas_price.iter().map(|obj|&obj.0.0).collect::<Vec<&GasPrice>>()
//     }

//     pub fn get_by_hash(&self, hash: &TxHash) -> Option<&Transaction> {
//         self.txs_store.get(hash)
//     }

//     pub fn get_best(&self, n: usize) -> Vec<Transaction> {
//      self.txs_by_gas_price
//      .iter()
//      .take(n)
//      .map(|obj| self.get_by_hash(&obj.1).unwrap().clone())
//      .collect::<Vec<Transaction>>() 
//     }

//     pub fn get_best_price(&self) -> Option<&GasPrice> {
//         self.txs_by_gas_price.iter().next().map(|obj| &obj.0.0)
//     }

//     pub fn remove_worst(&mut self) -> Option<Transaction>{
      
//         if let Some((price , hash)) = self.txs_by_gas_price.iter().next_back().cloned() {
//             self.txs_by_gas_price.remove(&(price , hash));
//             self.txs_store.remove(&hash)
//         } else {
//             None
//         }

//     }

// }