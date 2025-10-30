fn check_password(password: &str) -> Result<(), String> {
    if password.len() < 8 {
        Err(String::from("Password too short!"))
    } else if password.contains(' ') {
        Err(String::from("Password can't contain spaces!"))
    } else {
        Ok(())
    }
}

fn main() {
    let password = "abc123";
    match check_password(password) {
        Ok(()) => println!("Password accepted!"),
        Err(problem) => println!("Password rejected: {problem}"),
    }
}
