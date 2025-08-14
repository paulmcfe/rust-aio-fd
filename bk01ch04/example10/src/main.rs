fn main() {
    // Loop from 1 to 5 (inclusive)
    for number in 1..=5 {
        println!("Number: {number}");
    }
    
    // Loop from 0 to 4 (exclusive of 5)
    for number in 0..5 {
        println!("Index: {number}");
    }
    
    // Count backwards
    for number in (1..=5).rev() {
        println!("Countdown: {number}");
    }
}
