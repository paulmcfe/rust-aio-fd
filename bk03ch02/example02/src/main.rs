fn make_string_tuple(a: String, b: String) -> (String, String) {
    (a, b)
}

fn main() {
    let a = "Hello".to_string();
    let b = "world!".to_string();
    let pair = make_string_tuple(a, b);
    println!("{:?}", pair);
}
