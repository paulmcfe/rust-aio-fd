fn main() {
    // A temporary vector to hold the old wholesale prices
    let old_prices = vec![15.75, 22.2, 28.15, 30.6, 42.85];

    // Set up a vector for the new prices
    let new_prices: Vec<f64> = old_prices
        .into_iter()
        .map(|price| price * 1.05)
        .collect();

    // Print the new prices
    println!("{:.2?}", new_prices);

    // This won't work because old_prices is now invalid
    //println!("{:.2?}", old_prices);
}
