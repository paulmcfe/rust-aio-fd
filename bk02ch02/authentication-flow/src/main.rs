use std::io::{self, Write};

enum State {
    LoggedOut,
    EnteringName { name: String },
    LoggedIn { user: String },
}

fn prompt(label: &str) -> Result<String, io::Error> {
    print!("{label}");
    io::stdout().flush()?;
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    Ok(s.trim().to_string())
}

fn main() {
    let mut state = State::LoggedOut;

    loop {
        match &mut state {
            State::LoggedOut => {
                println!("Commands: login, quit");
                let cmd = prompt("> ").unwrap_or_default();
                match cmd.as_str() {
                    "login" => {
                        state = State::EnteringName {
                            name: String::new(),
                        }
                    }
                    "quit" => break,
                    _ => println!("Try 'login' or 'quit'."),
                }
            }
            State::EnteringName { name } => {
                if name.is_empty() {
                    *name = prompt("Enter your name: ").unwrap_or_default();
                } else {
                    println!("Welcome, {name}!");
                    state = State::LoggedIn { user: name.clone() };
                }
            }
            State::LoggedIn { user } => {
                println!("You are logged in as {user}.");
                println!("Commands: logout, whoami, quit");
                let cmd = prompt("> ").unwrap_or_default();
                match cmd.as_str() {
                    "logout" => state = State::LoggedOut,
                    "whoami" => println!("You are {user}."),
                    "quit" => break,
                    _ => println!("Try 'logout', 'whoami', or 'quit'."),
                }
            }
        }
    }

    println!("Goodbye!");
}
