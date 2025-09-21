use std::fmt::Display;

fn handle_data<T, U>(first: T, second: U) -> String
where
    T: Display + Clone + PartialOrd,
    U: Display + Default,
{
    format!("{} and {}", first, second)
}
fn main() {
    println!("{}", handle_data("Area", 51));
}
