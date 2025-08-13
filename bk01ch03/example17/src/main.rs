fn main() {
    let x = 20;
    let y = 20.0;

    // Integer examples
    println!("{}", x + 10);       // 30
    println!("{}", x / 8);        // 2
    println!("{}", x - 5 * 2);    // 10
    println!("{}", (x - 5) * 2);  // 30

    // Floating-point examples
    println!("{}", y * 5.0);      // 100
    println!("{}", y / 8.0);      // 2.5
    println!("{}", x as f32 * y); // 400
}
