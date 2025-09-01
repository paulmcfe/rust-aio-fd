fn main() {
    let string1 = String::from("Learning Rust adds ");
    let string2 = String::from("another string to your bow.");
    let backup_plan = string1.clone() + &string2;
    println!("{backup_plan}");
    //  Output: Learning Rust adds another string to your bow.
    println!("{string1}"); // This works!
}
