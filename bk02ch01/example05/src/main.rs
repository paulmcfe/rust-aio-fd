fn main() {
    let msg = "Important message".to_string();
    let length = get_length(msg);
    println!("{msg} is {length} characters long.");
}

fn get_length(s: String) -> usize {
    s.len()
}
