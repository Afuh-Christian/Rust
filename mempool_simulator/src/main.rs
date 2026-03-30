
mod modules;
use std::collections::BTreeSet;

use crate::modules::reverse_gas::ReverseGas;

// Example usage
fn main() {
    let mut set = BTreeSet::new();

    // Insert Gas Prices: 10, 50, 100
    set.insert((ReverseGas(10), "tx_low"));
    set.insert((ReverseGas(100), "tx_high"));
    set.insert((ReverseGas(50), "tx_med"));

    // Check the "Front" (First element)
    let first = set.iter().next().unwrap();
    
    // Because we used ReverseGas, the set is sorted: [100, 50, 10]
    // So the front is 100 (Highest Gas Price).
    println!("Front of set: {:?}", first.0); // Prints: ReverseGas(100)
}