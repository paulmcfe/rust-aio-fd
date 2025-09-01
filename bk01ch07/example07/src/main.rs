fn divide_if_positive(x: i32, y: i32) -> i32 {
    if y == 0 {
        return 0; // Early return to avoid division by zero
    }
    x / y
}
fn main() {
    let result = divide_if_positive(100, 0);
    println!("The result is {result}.")
}
