fn main() {
    let mut no_strings_attached = String::from("You see where this is going.");
    no_strings_attached.clear();
    println!("No strings attached? {}", no_strings_attached.is_empty());
    // Output:
    // No strings attached? true
}
