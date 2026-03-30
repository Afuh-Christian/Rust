use std::{cmp::Reverse, collections::{BTreeSet, HashMap}, hash::Hash};

use crate::modules::{reverse_gas::ReverseGas, transaction_model::Transaction, types::TxHash};

pub struct NaiveMempool {
    pub txs_store: HashMap<TxHash, Transaction>,
    pub txs_by_gas_price: BTreeSet<(TxHash , ReverseGas)>, // We will store gas prices in reverse order for easy retrieval of highest fee txs
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
        self.txs_by_gas_price.insert((tx.hash , ReverseGas(tx.gas_price)));
    }

    pub fn get_all(&self) -> Vec<Transaction> {
        self.txs_store.values().cloned().collect()
    }

    pub fn get_by_hash(&self, hash: &TxHash) -> Option<&Transaction> {
        self.txs_store.get(hash)
    }

    pub fn get_best(&self, n: usize) -> Vec<Transaction> {
     self.txs_by_gas_price
     .iter()
     .take(n)
     .map(|obj| self.get_by_hash(&obj.0).unwrap().clone())
     .collect::<Vec<Transaction>>() 
    }

}