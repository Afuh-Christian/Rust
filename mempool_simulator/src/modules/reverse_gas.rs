use std::{cmp::Ordering, collections::{BTreeSet, HashMap}};

use crate::modules::{transaction_model::Transaction, types::{GasPrice, TxHash}};

#[derive(Debug , Clone , PartialEq ,  Eq , PartialOrd , Hash)]
pub struct ReverseGas(pub GasPrice);

// to reverse the ordering . 

impl Ord for ReverseGas {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0) // 🔥 reversed
    }
}

// impl PartialOrd for ReverseGas {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }