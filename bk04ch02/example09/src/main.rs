use serde::{Deserialize, Serialize};
use std::fs;

// Need to derive the Serialize and Deserialize traits for a custom struct
#[derive(Debug, Serialize, Deserialize)]
struct GameState {
    player_name: String,
    level: u32,
    score: u64,
    items: Vec<String>,
}

// Save game state to a JSON file
fn save_game(save: &GameState) -> Result<(), Box<dyn std::error::Error>> {
    // Convert the game state to a JSON string
    let json = serde_json::to_string_pretty(save)?;

    // Write the JSON string to a file
    fs::write("savegame.json", json)?;
    println!("Game saved!");
    Ok(())
}

// Load a saved game state into a struct
fn load_game() -> Result<GameState, Box<dyn std::error::Error>> {
    // Read the JSON data as a String
    let json = fs::read_to_string("savegame.json")?;

    // Convert the JSON string to a GameState struct
    let save = serde_json::from_str(&json)?;

    // Return the GameState
    Ok(save)
}

fn main() {
    // Initialize the player's game state
    let game_state = GameState {
        player_name: String::from("Hero"),
        level: 42,
        score: 1337,
        items: vec![
            String::from("Sword of Debugging"),
            String::from("Shield of Error Handling"),
        ],
    };

    // Save the game state
    save_game(&game_state).expect("Failed to save");

    // Load and pattern-match the game state
    match load_game() {
        Ok(save) => println!(
            "Welcome back, {}! You're on level {}",
            save.player_name, save.level
        ),
        Err(e) => println!("Couldn't load save: {}", e),
    }
}
