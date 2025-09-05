use rand::Rng;

fn main() {
    // Create a random number generator
    let mut rng = rand::rng();

    // Roll a six-sided die
    let roll: u8 = rng.random_range(1..=6);

    println!("You rolled a {roll}!");
}
