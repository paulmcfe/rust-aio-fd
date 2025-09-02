fn main() {
    let mut total = 0;
    let mut add = |x| total += x; // Mutable borrow of total
    add(5);
    add(7);
    println!("{total}"); // Output: 12
}
