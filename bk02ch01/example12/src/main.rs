fn main() {
    let mut original = String::from("Original string"); // Mutable owner
    let immutable_share = &original; // Immutable share of the String
    println!("{immutable_share}"); // immutable_share is done after this
    let mutable_share = &mut original; // No problem now!
}
