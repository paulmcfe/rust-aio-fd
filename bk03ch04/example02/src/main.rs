fn get_first_element(vec: &[i32]) -> i32 {
    if vec.is_empty() {
        panic!("Return the first element of an empty vector? Get real.");
    }
    vec[0]
}

fn main() {
    let primes = vec![];
    let first = get_first_element(&primes);
    println!("The first element is {first}");
}
