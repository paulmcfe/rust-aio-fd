fn main() {
    let good_input = "42";
    let bad_input = "forty-two";

    // This is fine
    let number = good_input.parse::<u32>().unwrap();
    println!("The number is *drumroll*: {number}!");

    // This will panic if you uncomment it
    //let uhoh = bad_input.parse::<u32>().expect("Failed to parse number!");
}
