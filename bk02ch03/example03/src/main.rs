fn main() {
    let question = String::from("Why is there something rather than nothing?");

    let result1 = question.find("something");
    match result1 {
        Some(num) => println!("Substring found at position {num}!"),
        None => println!("Substring not found!"),
    }

    let result2 = question.find("nothinf");
    match result2 {
        Some(num) => println!("Substring found at position {num}!"),
        None => println!("Substring not found!"),
    }
}
