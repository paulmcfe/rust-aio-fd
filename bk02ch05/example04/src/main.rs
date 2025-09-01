fn main() {
    let mut sarcasm = String::from("World's tiniest");

    // Add single characters
    sarcasm.push(' ');
    sarcasm.push('🎻'); // Violin emoji
    sarcasm.push('!');
    println!("{sarcasm}");
}
