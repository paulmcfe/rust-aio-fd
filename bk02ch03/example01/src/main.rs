fn main() {
    let mut player_position = (125, 280);
    let x = player_position.0;
    let y = player_position.1;
    println!("The player is at position ({x}, {y}).");

    player_position.0 = 150;
    player_position.1 = 275;
    let (x, y) = player_position;
    println!("The player is now at position ({x}, {y}).");

    player_position.l
}
