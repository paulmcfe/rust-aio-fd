fn main() {
    let msg = "Important message".to_string();
    let str_length = get_length(msg);
    println!("'{msg}' is {str_length} characters long.");
}

fn get_length(s: String) -> usize {
    s.len()
}
