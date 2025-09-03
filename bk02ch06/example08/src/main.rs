fn main() {
    // A temporary vector to hold the old wholesale prices
    let old_prices = vec![15.75, 22.2, 28.15, 30.6, 42.85];

    // Set up a vector for the new prices
    let mut new_prices = Vec::new();

    // Iterate by taking ownership with into_iter()
    for price in old_prices.into_iter() {
        new_prices.push(price * 1.05);
    }

    // Print the new prices
    println!("{:.2?}", new_prices);

    // This won't work because old_prices is now invalid
    //println!("{:.2?}", old_prices);
}
