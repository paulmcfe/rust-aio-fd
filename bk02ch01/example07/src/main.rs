fn main() {
    let msg = "Important message".to_string(); // msg owns the String
    let str_length = get_length(&msg); // A reference to msg is passed
    println!("'{msg}' is {str_length} characters long."); // This works now!
}

fn get_length(s: &String) -> usize {
    // s borrows the passed value
    s.len()
}
