fn main() {
    let mut original = String::from("Original string"); // Mutable owner of String
    let mutable_share_1 = &mut original; // Mutable share of the String
    let mutable_share_2 = &mut original; // Not allowed!
    println!("{mutable_share_1}"); // mutable_share_1 is still alive here
}
