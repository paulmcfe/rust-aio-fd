use std::io;

fn main() {
    let mut input = String::new();
    println!("Type some text and then press Enter or Return:");
    io::stdin().read_line(&mut input).unwrap();
    println!("{}", input.trim());
}
