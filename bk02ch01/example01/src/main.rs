fn main() {
    {
        // A block scope starts here
        let y = 5; // y comes into scope and owns 5
        println!("The in-scope value of y is {y}.");
    } // The block scope ends here, so y's value is dropped
    println!("The out-of-scope value of y is {y}.");
}
