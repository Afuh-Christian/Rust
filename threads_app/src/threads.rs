use std::thread;
use std::sync::{Arc, Mutex};

pub fn sharing_data_safely_arc_mutex() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..4 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Final count = {}", *counter.lock().unwrap());
}


pub fn native_os_threads() {
    let handles: Vec<_> = (0..4).map(|i| {
        thread::spawn(move || {
            println!("Thread {} running", i);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}
