use std::env;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    for (index, arg) in arguments.iter().enumerate() {
        println!("Argument {index}: {arg}")
    }
}
