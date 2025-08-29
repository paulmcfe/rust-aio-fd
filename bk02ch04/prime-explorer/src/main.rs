use std::io::{self, Write};

fn main() {
    println!("Prime Explorer (vectors edition)");
    let n = prompt_for_limit();
    if n < 2 {
        println!("There are no primes ≤ {n}. Try a bigger number.");
        return;
    }

    // Core computation.
    let primes = sieve_primes(n);

    // All the analysis functions take slices, not owned vectors.
    let count = primes.len();
    let largest_gap = largest_prime_gap(&primes);
    let pct = (count as f64) / (n as f64) * 100.0;

    println!("Up to {n}, there are {count} prime numbers ({pct:.2}%).");

    if let Some((gap, a, b)) = largest_gap {
        println!("Largest gap is {gap} between {a} and {b}.");
    }

    // Show a small sample without flooding the terminal.
    println!("First few primes:");
    print_sample_prefix(&primes, 25);

    println!("Last few primes:");
    print_sample_suffix(&primes, 25);
}

/// Prompt the user for a positive integer limit.
/// Uses minimal String handling; the rest of the program is vector-focused.
fn prompt_for_limit() -> usize {
    loop {
        print!("Enter an upper limit (e.g., 100000): ");
        let _ = io::stdout().flush();

        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(_) => match line.trim().parse::<usize>() {
                Ok(n) => return n,
                Err(_) => {
                    eprintln!("Please enter a whole number like 100000.");
                }
            },
            Err(_) => eprintln!("Could not read input. Try again."),
        }
    }
}

/// Return a vector of all primes ≤ limit using the Sieve of Eratosthenes.
///
/// Implementation notes (vector muscle):
/// - A growable `Vec<bool>` tracks composite vs prime.
/// - We allocate once with `vec![true; ...]`.
/// - We collect primes into a second `Vec<usize>` via `push`.
fn sieve_primes(limit: usize) -> Vec<usize> {
    // Start by assuming every number is a prime
    let mut is_prime = vec![true; limit + 1];

    // 0 and 1 are not prime
    is_prime[0] = false;
    if limit >= 1 {
        is_prime[1] = false;
    }

    // Mark off prime multiples starting at p*p and stepping by p
    // For each multiple m, is_prime[m] is set to false (not prime)
    let mut p = 2;
    while p * p <= limit {
        if is_prime[p] {
            let mut m = p * p;
            while m <= limit {
                is_prime[m] = false;
                m += p;
            }
        }
        p += 1;
    }

    // At this point, is_prime[i] is true for every i that's prime
    // So, now we collect those prime indices into a separate vector
    let mut primes = Vec::new();
    let mut i = 2;
    while i <= limit {
        if is_prime[i] {
            primes.push(i);
        }
        i += 1;
    }
    primes
}

/// Find the largest gap between consecutive primes in a *sorted* primes slice.
/// Returns (gap_size, lower_prime, upper_prime) or None if not enough primes.
fn largest_prime_gap(primes: &[usize]) -> Option<(usize, usize, usize)> {
    if primes.len() < 2 {
        return None;
    }
    let mut best_gap = 0usize;
    let mut best_a = primes[0];
    let mut best_b = primes[1];

    // Manual index loop (shows off indexing without iterator combinators).
    let mut i = 1usize;
    while i < primes.len() {
        let a = primes[i - 1];
        let b = primes[i];
        let gap = b - a;
        if gap > best_gap {
            best_gap = gap;
            best_a = a;
            best_b = b;
        }
        i += 1;
    }
    Some((best_gap, best_a, best_b))
}

/// Print up to `count` values from the start of the slice.
fn print_sample_prefix(items: &[usize], count: usize) {
    let end = if items.len() < count {
        items.len()
    } else {
        count
    };
    if end == 0 {
        println!("    (none)");
        return;
    }
    let mut i = 0;
    while i < end {
        print!("{:>6}", items[i]);
        if (i + 1) % 10 == 0 || i + 1 == end {
            println!();
        }
        i += 1;
    }
}

/// Print up to `count` values from the end of the slice.
fn print_sample_suffix(items: &[usize], count: usize) {
    if items.is_empty() {
        println!("    (none)");
        return;
    }
    let len = items.len();
    let start = if len > count { len - count } else { 0 };
    let mut i = start;
    while i < len {
        print!("{:>6}", items[i]);
        if (i - start + 1) % 10 == 0 || i + 1 == len {
            println!();
        }
        i += 1;
    }
}
