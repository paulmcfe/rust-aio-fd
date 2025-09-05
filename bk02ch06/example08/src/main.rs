fn main() {
    // Here's the collection
    let primes = vec![2, 3, 5, 7, 11, 13];

    // Iterate with an immutable borrow via iter()
    for prime in primes.iter() {
        print!("{}, ", prime); // Each prime is &i32
    }

    // The primes vector is still valid here
    println!("\n{:?}", primes);
}
