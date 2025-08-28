use ::std::num::ParseIntError;

fn parse_age(text: &str) -> Result<u8, ParseIntError> {
    let n = text.trim().parse::<u8>()?;
    Ok(n)
}

fn main() {
    match parse_age("42") {
        Ok(age) => println!("Age is {age}."),
        Err(e) => println!("Couldn't parse age: {e}."),
    }
    match parse_age("forty-two") {
        Ok(age) => println!("Age is {age}."),
        Err(e) => println!("Couldn't parse age: {e}."),
    }
}
