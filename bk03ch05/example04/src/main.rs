macro_rules! shout {
    ($word:expr) => {
        println!("{}!!!", $word.to_uppercase());
    };
}

fn main() {
    shout!("help"); // Prints: HELP!!!
}
