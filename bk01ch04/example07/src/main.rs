fn main() {
    let mut num = 2;
    loop {
        let num_squared = num * num;
        println!("{num} squared = {num_squared}");
        num += 1;
        if num > 10 {
            break;
        }
    }
}
