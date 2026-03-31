use std::{cmp::Reverse, collections::{BTreeSet, HashMap}, hash::Hash};

use crate::modules::{reverse_gas::ReverseGas, transaction_model::Transaction, types::{GasPrice, TxHash}};

pub struct NaiveMempool {
    pub txs_store: HashMap<TxHash, Transaction>,
    pub txs_by_gas_price: BTreeSet<(ReverseGas , TxHash)>, // We will store gas prices in reverse order for easy retrieval of highest fee txs
}

impl NaiveMempool {
    pub fn new() -> Self {
        NaiveMempool {
            txs_store: HashMap::new(),
            txs_by_gas_price: BTreeSet::new(),
        }
    }

    pub fn add(&mut self, tx: &Transaction) {
        self.txs_store.insert(tx.hash , tx.clone());
        self.txs_by_gas_price.insert((ReverseGas(tx.gas_price), tx.hash));
    }

    pub fn get_all(&self) -> Vec<Transaction> {
        self.txs_store.values().cloned().collect()
    }

      pub fn get_price_hash_tree(&self) -> Vec<&ReverseGas> {
        self.txs_by_gas_price.iter().map(|obj| &obj.0).collect::<Vec<&ReverseGas>>()
    }

      pub fn get_price_tree(&self) -> Vec<&GasPrice> {
        self.txs_by_gas_price.iter().map(|obj|&obj.0.0).collect::<Vec<&GasPrice>>()
    }

    pub fn get_by_hash(&self, hash: &TxHash) -> Option<&Transaction> {
        self.txs_store.get(hash)
    }

    pub fn get_best(&self, n: usize) -> Vec<Transaction> {
     self.txs_by_gas_price
     .iter()
     .take(n)
     .map(|obj| self.get_by_hash(&obj.1).unwrap().clone())
     .collect::<Vec<Transaction>>() 
    }

    pub fn get_best_price(&self) -> Option<&GasPrice> {
        self.txs_by_gas_price.iter().next().map(|obj| &obj.0.0)
    }

    pub fn remove_worst(&mut self) -> Option<Transaction>{
      
        if let Some((price , hash)) = self.txs_by_gas_price.iter().next_back().cloned() {
            self.txs_by_gas_price.remove(&(price , hash));
            self.txs_store.remove(&hash)
        } else {
            None
        }

    }

}