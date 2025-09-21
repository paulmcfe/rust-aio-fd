
fn array_contains<T: PartialEq>(slice: &[T], item: T) -> bool {
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

    let fruit = ["apple", "banana", "grape", "peach", "pear"];
    println!("{}", array_contains(&fruit, "cherry"));
}
