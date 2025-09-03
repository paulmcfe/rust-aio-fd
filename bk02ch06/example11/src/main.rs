fn main() {
    let rates = vec![0.45, 0.73, 0.33, 1.15, 0.9, 2.76, 3.25, 1.55];

    let result = rates.iter().find(|&n| *n > 1.0);
    match result {
        Some(first_over_one) => println!("The first rate over 1.0 is {first_over_one}"),
        None => println!("No rate over 1.0 found!"),
    }
}
