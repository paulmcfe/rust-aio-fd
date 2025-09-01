fn main() {
    let x = 10;
    let y = 15;
    let z = 20;

    println!("{}", x < y && x < z); // true
    println!("{}", y > x || y > z); // true
    println!("{}", !(x < z)); // false
}
