fn process_inventory(items: Vec<(&str, i32)>) -> i32 {
    let mut total = 0;

    for (item, count) in items {
        println!("Processing {} (count: {})", item, count);

        if count < 0 {
            println!("    WARNING: Negative count for {}", item);
            continue;
        }

        total += count;
        println!("    Running total: {}", total)
    }
    total
}

fn main() {
    let inventory = vec![
        ("apples", 5),
        ("bananas", -2), // Oops!
        ("oranges", 3),
    ];

    let total = process_inventory(inventory);
    println!("Final total: {}", total);
}
