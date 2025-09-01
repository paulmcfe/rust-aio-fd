fn main() {
    let string1 = String::from("puppet");
    let string2 = String::from("lonely");
    let string3 = String::from("string");
    let lyric = format!("Just a {string1} on a {string2} {string3}.");
    println!("{lyric}"); // Output: Just a puppet on a lonely string.
}
