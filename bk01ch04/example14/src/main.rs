fn main() {
    'outer: for i in 1..=3 {
        for j in 1..=3 {
            if i * j == 4 {
                println!("Found 4 at i={i}, j={j}.");
                break 'outer; // break the outer loop, not just the inner one
            }
        }
    }
}
