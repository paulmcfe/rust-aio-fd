use std::time::Instant;

macro_rules! timer {
    ( $block:block ) => {
        {
            // Start the timer
            let start = Instant::now();

            // Execute the block of code
            $block

            // Stop the clock!
            let duration = start.elapsed();
            println!("Code block execution time: {:.2?}", duration);
        }
    };
}

// Simple functions to demonstrate timing different algorithms

// Calculate sum using a simple loop
fn sum_iterative(n: u32) -> u64 {
    let mut total = 0;
    for i in 1..=n {
        total += i as u64;
    }
    total
}

// Calculate sum using the mathematical formula: n * (n + 1) / 2
fn sum_formula(n: u32) -> u64 {
    let n = n as u64;
    n * (n + 1) / 2
}

fn main() {
    timer!({
        let result = sum_iterative(1_000_000);
        println!("Sum using loop: {}", result);
    });

    timer!({
        let result2 = sum_formula(1_000_000);
        println!("\nSum using formula: {}", result2);
    });
}
