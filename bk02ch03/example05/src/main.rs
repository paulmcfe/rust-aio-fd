fn main() {
    // The original string
    let question = String::from("Why is there something rather than nothing?");

    // Search for the substring
    let result = question.find("something");

    // Use the result to set the start of the slice
    // (0 if the substring wasn't found)
    let start = result.unwrap_or(0);

    // Display the slice
    println!("Slice: {}", &question[start..]);
}
