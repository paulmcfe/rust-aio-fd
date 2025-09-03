fn main() {
    // A collection of boastful words
    let mut words = vec![
        String::from("BLUSTER"),
        String::from("BRAGGADOCIO"),
        String::from("GASCONADE"),
        String::from("RODOMONTADE"),
    ];

    // Iterate with a mutable borrow via iter_mut()
    for word in words.iter_mut() {
        // Lowercase the word
        word.make_ascii_lowercase(); // Each word is &mut String
    }

    // The words vector is still valid here
    // but the words have been changed
    println!("{:?}", words);
}
