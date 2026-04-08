
mod modules;
use crate::modules::{naive_mempool::NaiveMempool, transaction_model::{Transaction, generate_tx}, types::GasPrice};

// Example usage
fn main() {
    let mempool = NaiveMempool::new(); 

    let transactions = (1..10).map(|i| generate_tx(i)).collect::<Vec<Transaction>>();
    transactions.iter().for_each(|tx| mempool.add(tx));



    tokio::spawn(async move {
        loop {
            let all_txs = mempool.get_all();
            println!("Current Mempool Transactions: {:?}", all_txs);
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    });


}

