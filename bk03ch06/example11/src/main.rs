use rayon::prelude::*;

fn main() {
    let numbers: Vec<u64> = (1..=1_000_000).collect();

    // Sequential sum
    let sequential_sum: u64 = numbers.iter().sum();
    println!("Sequential sum is {}", sequential_sum);

    // Parallel sum
    let parallel_sum: u64 = numbers.par_iter().sum();
    println!("Parallel sum is {}", parallel_sum);
}
