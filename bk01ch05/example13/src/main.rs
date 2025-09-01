fn main() {
    // Creating a simple multiplication table
    for i in 1..=10 {
        for j in 1..=10 {
            print!("{:3} ", i * j);
        }
        println!(); // New line after each row
    }
}