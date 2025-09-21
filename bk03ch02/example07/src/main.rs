fn array_contains(slice: &[i32], item: i32) -> bool {
    for value in slice {
        if *value == item {
            return true;
        }
    }
    false
}

fn main() {
    let primes = [2, 3, 5, 7, 11];
    println!("{}", array_contains(&primes, 3));
}