use std::thread;

fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5, 6];

    println!("Before processing: {:?}", numbers);

    // Create a scope for our threads
    thread::scope(|s| {
        // You can borrow `numbers` from the main thread here
        let mid = numbers.len() / 2;

        // Split `numbers`` into two parts
        let (part1, part2) = numbers.split_at_mut(mid);

        // Spawn a thread to process the first part
        s.spawn(|| {
            println!("Worker #1 started.");
            for num in part1 {
                *num *= 2; // Double each number
            }
            println!("Worked #1 finished.")
        });

        // Spawn another thread to process the second part
        s.spawn(|| {
            println!("Worker #2 started.");
            for num in part2 {
                *num += 10; // Add 10 to each number
            }
            println!("Worked #2 finished.")
        });

        // The scope automatically waits here for both threads to complete
    });
    println!("After processing: {:?}", numbers);
}
