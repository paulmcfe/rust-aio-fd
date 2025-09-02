fn main() {
    let threshold = 1000;
    let is_big = |n| n >= threshold; // Immutable borrow of threshold
    println!("{}", is_big(1001)); // Output: true
}
