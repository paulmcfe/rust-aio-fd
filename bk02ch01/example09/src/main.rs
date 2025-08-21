fn main() {
    let mut original = String::from("Original string"); // Mutable owner of String
    let immutable_share = &original; // Immutable share of the String
    let mutable_share = &mut original; // Not allowed!
    println!("{immutable_share}"); // immutable_share is still alive here
}
