fn main() {
    let mut num = 2;
    let result = loop {
        let num_squared = num * num;
        println!("{num} squared = {num_squared}");

        if num_squared > 50 {
            break num
        }
        
        num += 1;
    };
    println!("The first number with a square larger than 50 is {result}.");
}
