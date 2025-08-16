fn factorial(n: u128) -> u128 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    let number = 5;
    println!("factorial({number}) = {}", factorial(number));
}

