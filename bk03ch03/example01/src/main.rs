#[allow(unused_variables)]

fn main() {
    let mut original = String::from("Original string");
    let ref1 = &original; // The ref1 lifetime starts here
    println!("{ref1}"); // ref1 is not used after this, so its lifetime ends
    let ref2 = &mut original; // No problem

}
