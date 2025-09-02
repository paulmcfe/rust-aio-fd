fn main() {
    // No types - let Rust figure them out
    //let complement1 = |angle| angle + 180;

    // Specify the parameter type
    let complement2 = |angle: i32| angle + 180;

    // Specify the parameter type and the return type
    let complement3 = |angle: i32| -> i32 { angle + 180 };

    // Use an expression block
    let println!("{}", complement2(30));
}
