fn main() {
    let n = 15;

    match n {
        _ if n % 15 == 0 => println!("FizzBuzz"),
        _ if n % 3 == 0  => println!("Fizz"),
        _ if n % 5 == 0  => println!("Buzz"),
        _                => println!("{n}"),
    }
}
