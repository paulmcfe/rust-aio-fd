use std::fs;

fn read_shopping_list() -> std::io::Result<()> {
    // Read entire file into a String
    let contents = fs::read_to_string("shopping-list.txt")?;
    
    // Print the contents line by line
    println!("Shopping list:");
    for line in contents.lines() {
        println!("  □ {}", line);
    }
    
    Ok(())
}

fn main() {
    match read_shopping_list() {
        Ok(_) => println!("\nDon't forget your reusable bags!"),
        Err(e) => println!("Couldn't read list: {}. Guess we're ordering pizza.", e),
    }
}