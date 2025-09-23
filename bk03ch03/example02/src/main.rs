fn main() {
    let s = String::from("hey");
    let r = &s;  // The lifetime of r starts here
    let shout_it = || println!("{}!", r.to_uppercase()); // Last use of r
    shout_it();  // Doesn't matter, though: r must live until the closure is done
}
