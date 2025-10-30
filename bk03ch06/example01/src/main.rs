use std::thread;
use std::time::Duration;

fn main() {
    println!("Starting main thread.");

    // Spawn a new thread to do some "heavy" work
    let handle = thread::spawn(|| {
        println!("Starting spawned thread.");

        // Sleep for two seconds to simulate work
        thread::sleep(Duration::from_secs(2));

        // This statement doesn't run until two seconds later
        println!("Spawned thread finished.");

        // Return a value from the thread
        42
    });

    // The main thread does other stuff while the spawned thread is busy
    println!("Main thread is doing other things.");

    // Sleep for 2500ms to simulate work
    thread::sleep(Duration::from_millis(2500));

    // This statement doesn't run until 2500ms later
    println!("Main thread is ready to wait.");

    // Wait for the spawned thread to finish
    if let Ok(result) = handle.join() {
        println!("Worker thread returned: {result}");
    }
}
