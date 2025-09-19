fn make_i32_tuple(a: i32, b: i32) -> (i32, i32) {
    (a, b)
}

fn main() {
    let a = 5;
    let b = 10;
    let pair = make_i32_tuple(a, b);
    println!("{:?}", pair);
}
