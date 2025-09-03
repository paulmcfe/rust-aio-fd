fn main() {
    let numbers: Vec<i32> = vec![4, 6, 3, 2, 1, 5];

    let evens: Vec<&i32> = numbers
        .iter()
        .filter(|&n| *n % 2 == 0)
        .collect();
    println!("{:?}", evens);
}
