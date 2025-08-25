#![allow(unused_variables)]
enum PlayerState {
    Active(String), // Store the player's name
    Idle(u32),      // Stores the idle time, in seconds
    Dead(bool),     // true = can respawn; false = permanently dead
}

fn main() {
    let state1 = PlayerState::Active(String::from("Alice"));
    let state2 = PlayerState::Idle(120);
    let state3 = PlayerState::Dead(true);

    match state1 {
        PlayerState::Active(name) => println!("{name} is playing!"),
        PlayerState::Idle(secs) => println!("Player idle for {secs} seconds."),
        PlayerState::Dead(can_respawn) => {
            if can_respawn {
                println!("Player is dead but can respawn.");
            } else {
                println!("Player is permanently dead.");
            }
        }
    }
}
