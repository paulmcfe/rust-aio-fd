use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct GameSave {
    player_name: String,
    level: u32,
    score: u64,
    items: Vec<String>,
}

fn save_game(save: &GameSave) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(save)?;
    fs::write("savegame.json", json)?;
    println!("Game saved!");
    Ok(())
}

fn load_game() -> Result<GameSave, Box<dyn std::error::Error>> {
    let json = fs::read_to_string("savegame.json")?;
    let save = serde_json::from_str(&json)?;
    Ok(save)
}

fn main() {
    let game_state = GameSave {
        player_name: String::from("Hero"),
        level: 42,
        score: 1337,
        items: vec![
            String::from("Sword of Debugging"),
            String::from("Shield of Error Handling"),
        ],
    };
    
    save_game(&game_state).expect("Failed to save");
    
    match load_game() {
        Ok(save) => println!("Welcome back, {}! You're level {}", save.player_name, save.level),
        Err(e) => println!("Couldn't load save: {}", e),
    }
}