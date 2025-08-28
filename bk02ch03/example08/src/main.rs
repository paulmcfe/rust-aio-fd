use std::io;

fn main() {
    println!("Enter a red component value (0 - 255):");
    let mut choice = String::new();
    let _ = io::stdin().read_line(&mut choice);
    let choice: Result<u8, _> = choice.trim().parse();
    let color = choice.unwrap_or(255);
    println!("Your red component value is {color}.");
}
