const APP_NAME: &str = "The Global Game";
const MAX_PLAYERS: u16 = 100;

fn initialize() {
    // MAX_PLAYERS has global scope, so it can be used here
    println!("The maximum number of players is {MAX_PLAYERS}.");
}

fn main() {
    // APP_NAME has global scope, so it can be used here
    println!("Welcome to {APP_NAME}!");
    initialize();
}
