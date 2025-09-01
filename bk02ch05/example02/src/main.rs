fn main() {
    let english = String::from("Symphony"); // 8 English characters
    let russian = String::from("Симфония"); // 8 Cyrillic characters
    let korean = String::from("교향곡"); // 3 Korean characters
    let emoji = String::from("🎼🎵🎻🎶"); // 4 emoji characters

    println!("English string has {} characters.", english.chars().count()); // 8
    println!("Russian string has {} characters.", russian.chars().count()); // 8
    println!("Korean string has {} characters.", korean.chars().count());   // 3
    println!("Emoji string has {} characters.", emoji.chars().count());   // 4
}
