// Function that reads the number of items in stock from a config file
fn read_stock_count(item_name: &str) -> Result<i32, String> {
    // Simulate reading from a config file that might not exist or be corrupted
    match item_name {
        "apples" => Ok(50),
        "bananas" => Ok(25),
        _ => Err("Item not found in inventory".to_string()),
    }
}

fn main() {
    // Get stock count for various items, defaulting to 0 if there's an error
    let apple_stock = read_stock_count("apples").unwrap_or_default();
    let orange_stock = read_stock_count("oranges").unwrap_or_default(); // Will return 0

    println!("Apples in stock: {}", apple_stock); // 50
    println!("Oranges in stock: {}", orange_stock); // 0 (makes sense - no stock if not found)
}
