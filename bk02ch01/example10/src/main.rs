fn main() {
    let mut original = String::from("Original string"); // Mutable owner of String
    let mutable_share = &mut original; // Mutable share of the String
    let immutable_share = &original; // Not allowed!
    println!("{mutable_share}"); // mutable_share is still alive here
}
