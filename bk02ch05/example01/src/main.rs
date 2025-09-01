fn main() {
    let english = String::from("Symphony"); // 8 English characters
    let russian = String::from("Симфония"); // 8 Cyrillic characters
    let korean = String::from("교향곡"); // 3 Korean characters
    let emoji = String::from("🎼🎵🎻🎶"); // 4 emoji characters

    println!("English string uses {} bytes.", english.len()); // 8 bytes
    println!("Russian string uses {} bytes.", russian.len()); // 16 bytes
    println!("Korean string uses {} bytes.", korean.len());   // 9 bytes
    println!("Emoji string uses {} bytes.", emoji.len());   // 16 bytes
}
