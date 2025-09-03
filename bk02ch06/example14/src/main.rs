fn main() {
    let fruits = vec![
        "apple",
        "banana",
        "cherry",
        "peach",
        "grape",
        "blueberry",
        "mandarin",
        "blackberry",
        "strawberry",
        "pear",
    ];

    fruits
        .iter()
        .enumerate()
        .for_each(|(i, fruit)| println!("{}: {}", i + 1, fruit));
}
