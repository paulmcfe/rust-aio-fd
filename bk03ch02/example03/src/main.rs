fn make_tuple<T>(a: T, b: T) -> (T, T) {
    (a, b)
}
fn main() {
    let a = 5;
    let b = 10;
    let pair = make_tuple(a, b);
    println!("{:?}", pair);

    let a = "Hello".to_string();
    let b = " world!".to_string();
    let pair = make_tuple(a, b);
    println!("{:?}", pair);
}
