use std::thread;

// #[tokio::main(flavor = "multi_thread", worker_threads = 4)]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    for i in 0..8 {
        tokio::spawn(async move {
            let tid = thread::current().id();
            println!("Task {} running on thread {:?}", i, tid);

            tokio::time::sleep(std::time::Duration::from_secs(5)).await;

            let tid2 = thread::current().id();
            println!("Task {} resumed on thread {:?}", i, tid2);
        });
    }

    println!("----");
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;

      println!("--end--");
}
