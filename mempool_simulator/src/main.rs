
mod modules;
use crate::modules::{naive_mempool::NaiveMempool, transaction_model::{Transaction, generate_tx}, types::GasPrice};

// Example usage
fn main() {
    let mut mempool = NaiveMempool::new(); 
    let tx1 = generate_tx(1);
    let tx2 = generate_tx(2);
    let tx3 = generate_tx(3);
    let tx4 = generate_tx(4);
    let tx5 = generate_tx(5);
    mempool.add(&tx1);
    mempool.add(&tx2);
    mempool.add(&tx3);
    mempool.add(&tx4);
    mempool.add(&tx5);


  println!("All transactions in mempool : {:?}", mempool.get_all().iter().map(|tx| &tx.gas_price).cloned().collect::<Vec<GasPrice>>() );
  println!("Best price in mempool : {:?}", mempool.get_best_price().unwrap());
  println!("Best transaction in mempool : {:?}", mempool.get_best(2).iter().map(|tx| &tx.gas_price).collect::<Vec<&GasPrice>>() );

   mempool.remove_worst(); 

    println!("All transactions in mempool : {:?}", mempool.get_all().iter().map(|tx| &tx.gas_price).collect::<Vec<&GasPrice>>() );
}

