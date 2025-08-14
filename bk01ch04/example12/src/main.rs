fn main() {
    for num in 2..=10 {
        // Skip odd numbers
        if num % 2 != 0 {
            continue;
        }
        let num_squared = num * num;
        println!("{num} squared = {num_squared}");
    }
}
