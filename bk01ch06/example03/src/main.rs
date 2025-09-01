fn main() {
    let age = 25;
    
    match age {
        0..=12 => println!("Child"),
        13..=19 => println!("Teenager"),
        20..=64 => println!("Adult"),
        65..=120 => println!("Senior"),
        _ => println!("That's not a realistic age!"),
    }
}
