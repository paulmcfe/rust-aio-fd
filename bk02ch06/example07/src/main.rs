fn main() {
    // Concise version - Let the compiler figure it out
    let square = |x| x * x;
    println!("{}", square(10)); // Output: 100

    // Version 2: Add a type to the closure argument
    let square2 = |x: i32| x * x;
    println!("{}", square2(10)); // Output: 100

    // Version 3: Add types to the closure argument and result
    let square3 = |x: i32| -> i32 { x * x };
    println!("{}", square3(10)); // Output: 100

    // Expression block example
    let hypotenuse_length = |a, b| {
        let sum_of_squares: f64 = square(a) as f64 + square(b) as f64;
        sum_of_squares.sqrt()
    };
    println!("{}", hypotenuse_length(3, 4)); // Output: 5
}