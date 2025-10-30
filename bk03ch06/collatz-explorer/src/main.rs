use rayon::prelude::*;
use std::time::Instant;

fn collatz_length(mut n: u64) -> u32 {
    let mut steps = 0;

    while n != 1 {
        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = n * 3 + 1;
        }
        steps += 1;
    }
    steps
}

fn collatz_chain(mut n: u64) {
    print!("{n}");

    while n != 1 {
        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = n * 3 + 1;
        }
        print!(" -> {}", n);
    }
}

fn find_longest_sequential(start: u64, end: u64) -> (u64, u32) {
    let mut longest_n = start;
    let mut longest_length = 0;

    for n in start..=end {
        let length = collatz_length(n);
        if length > longest_length {
            longest_length = length;
            longest_n = n;
        }
    }
    (longest_n, longest_length)
}

fn find_longest_parallel(start: u64, end: u64) -> (u64, u32) {
    (start..=end)
        .into_par_iter()
        .map(|n| (n, collatz_length(n)))
        .max_by_key(|&(_, length)| length)
        .unwrap()
}

fn main() {
    println!("Longest Collatz Sequence Finder");
    println!("{}", "=".repeat(50));
    println!("Finding the number with the longest journey to 1...\n");

    // Test with increasingly large ranges
    let ranges = vec![(1, 10_000), (10_000, 100_000), (100_000, 1_000_000)];

    for (start, end) in ranges {
        println!("Range: {} to {}", start, end.to_string().as_str());
        println!("{}", "-".repeat(30));

        // Sequential
        let start_time = Instant::now();
        let (longest_seq, length_seq) = find_longest_sequential(start, end);
        let time_seq = start_time.elapsed();

        println!("Sequential:");
        println!(
            "  The number {} produces the longest chain ({} steps)",
            longest_seq, length_seq
        );
        println!("  Time: {:.3}s", time_seq.as_secs_f64());

        // Parallel
        let start_time = Instant::now();
        let (longest_par, length_par) = find_longest_parallel(start, end);
        let time_par = start_time.elapsed();

        println!("Parallel:");
        println!(
            "  The number {} produces the longest chain ({} steps)",
            longest_par, length_par
        );
        println!("  Time: {:.3}s", time_par.as_secs_f64());

        println!(
            "  Speedup: {:.2}x\n",
            time_seq.as_secs_f64() / time_par.as_secs_f64()
        );
    }

    // Show the actual sequence for a few numbers
    println!("{}", "=".repeat(50));
    println!("Here are the Collatz chains produces by the first ten integers:\n");
    for n in 1..=10 {
        collatz_chain(n);
        println!();
    }

    // Show the actual sequence for a famous number
    println!("{}", "=".repeat(50));
    println!("Let's trace the journey of 27:\n");
    collatz_chain(27u64);

    println!(
        "\n\nThat's {} steps! Some numbers take surprisingly long journeys.",
        collatz_length(27)
    );
}
