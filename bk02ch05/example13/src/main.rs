fn main() {
    let advice = "Time to cut the apron strings and move out!";
    println!("{}", &advice[..5]);    // Time
    println!("{}", &advice[16..29]); // apron strings
    println!("{}", &advice[34..]);   // move out!
}
