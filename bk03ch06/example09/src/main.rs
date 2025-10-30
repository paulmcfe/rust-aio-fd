use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Create an Arc smart pointer to some heap data
    // and protect it with a Mutex
    let shared_log = Arc::new(Mutex::new(vec![]));

    // Create a vector to hold the spawned thread handles
    let mut handles = vec![];

    // Loop to spawn some threads
    for i in 0..3 {
        // Clone the Arc to give shared ownership to the upcoming thread
        let log_clone = Arc::clone(&shared_log);

        // Spawn a thread
        let handle = thread::spawn(move || {
            // Acquire the lock to get mutable access to the data
            let mut log = log_clone.lock().unwrap();
            log.push(format!("Event from thread {i}"));
            // Lock automatically released here when `log` goes out of scope
        });
        handles.push(handle);
    }

    // Wait for all the threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Acquire the lock to print the final shared log
    println!("Final log:\n{:?}", shared_log.lock().unwrap());
}
