mod modules;

use std::sync::Arc;
use std::time::Instant;
use tokio::time::{sleep, Duration};
use rayon::prelude::*;

use crate::modules::{
    naive_mempool::NaiveMempool,
    transaction_model::generate_tx,
};

#[tokio::main]
async fn main() {
    let mempool = Arc::new(NaiveMempool::new());

    // --- Producer Tasks ---
    // spawn_blocking keeps CPU-heavy rayon work off the async runtime
    for i in 0..4 {
        let mempool_producer = mempool.clone();
        tokio::spawn(async move {
            let mut nonce = (i as u64) * 10_000_000;
            loop {
                let batch = {
                    let nonce_start = nonce;
                    tokio::task::spawn_blocking(move || {
                        (nonce_start..nonce_start + 1000)
                            .into_par_iter()
                            .map(|n| generate_tx(n))
                            .collect::<Vec<_>>()
                    })
                    .await
                    .unwrap()
                };

                for tx in &batch {
                    mempool_producer.add(tx);
                }

                nonce += 1000;
                // yield so other async tasks (miner, metrics) get scheduled
                tokio::task::yield_now().await;
            }
        });
    }

    // --- Miner Task ---
    let mempool_miner = mempool.clone();
    tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(2)).await;
            let mined = mempool_miner.mine_block(21_000 * 500);
            if !mined.is_empty() {
                println!("[MINER] ⛏  Mined {} transactions", mined.len());
            }
        }
    });

    // --- Metrics Task ---
    let mempool_metrics = mempool.clone();
    tokio::spawn(async move {
        use std::sync::atomic::Ordering::Relaxed;

        let mut last_added = 0u64;
        let mut last_time = Instant::now();

        loop {
            sleep(Duration::from_secs(1)).await;

            let now = Instant::now();
            let elapsed = now.duration_since(last_time).as_secs_f64();
            last_time = now;

            let total_added    = mempool_metrics.total_added.load(Relaxed);
            let total_mined    = mempool_metrics.total_mined.load(Relaxed);
            let total_evicted  = mempool_metrics.total_evicted.load(Relaxed);
            let total_rejected = mempool_metrics.total_rejected.load(Relaxed);
            let pool_size      = mempool_metrics.txs_store.len();

            let tps = (total_added - last_added) as f64 / elapsed;
            last_added = total_added;

            println!(
                "[METRICS] TPS: {:>8.0} | pool: {:>6} | added: {:>8} | mined: {:>6} | evicted: {:>6} | rejected: {:>6}",
                tps, pool_size, total_added, total_mined, total_evicted, total_rejected
            );
        }
    });

    loop {
        sleep(Duration::from_secs(60)).await;
    }
}