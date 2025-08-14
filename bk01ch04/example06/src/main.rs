fn main() {
    let mut num = 2;
    let mut num_squared = num * num;
    println!("{num} squared = {num_squared}");

    num += 1;
    num_squared = num * num;
    println!("{num} squared = {num_squared}");

    num += 1;
    num_squared = num * num;
    println!("{num} squared = {num_squared}");
}
