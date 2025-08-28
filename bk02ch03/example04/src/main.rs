fn main() {
    // The goal: sum the numbers in this string
    let code = String::from("398w4x881b");

    // Initialize the sum
    let mut num_sum = 0;

    // Loop through all the characters in the string
    for c in code.chars() {
        // Try to convert the char c to a digit
        let result: Option<u32> = c.to_digit(10);

        // We only care if the Option contains a value
        if let Some(digit) = result {
            num_sum += digit;
        }
    }
    println!("The sum of the digits is {num_sum}");
}
