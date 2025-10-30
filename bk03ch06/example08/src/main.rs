use std::sync::Arc;
use std::thread;

fn main() {
    let original = vec![1, 2, 3, 4, 5];

    // Move the vector into a new Arc
    let data = Arc::new(original);
    println!(
        "Original Arc reference count: {}\n",
        Arc::strong_count(&data)
    );

    // Clone the Arc safely to share the vector across threads
    println!("Cloning Arc pointer...");
    let clone1 = Arc::clone(&data);
    println!("Reference count is now: {}\n", Arc::strong_count(&data));

    println!("Cloning Arc pointer...");
    let clone2 = Arc::clone(&data);
    println!("Reference count is now: {}\n", Arc::strong_count(&data));

    // Spawn thread #1
    let handle1 = thread::spawn(move || {
        let product: i32 = clone1.iter().product();
        println!("Thread #1 using data to calculate product: {product}");
    });

    // Close thread #1
    handle1.join().unwrap();
    println!("Thread #1 complete.");
    println!("Reference count is now: {}\n", Arc::strong_count(&data));

    // Spawn thread #2
    let handle2 = thread::spawn(move || {
        let sum: i32 = clone2.iter().sum();
        println!("Thread #2 using data to calculate sum: {sum}");
    });

    // Close thread #2
    handle2.join().unwrap();
    println!("Thread #2 complete.");
    println!("Reference count is now: {}\n", Arc::strong_count(&data));
}
