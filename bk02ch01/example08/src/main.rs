fn main() {
    let mut warn = String::from("Careful"); // warn owns the String mutably
    let shout = &mut warn; // shout is a mutable borrow of warn
    shout.push_str("!!!"); // shout can modify the original String
    println!("{warn}"); // Output: Careful!!!
}
