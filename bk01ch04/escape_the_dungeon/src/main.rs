use std::io;

fn main() {
    println!("=== Escape the Dungeon ===");
    println!("You wake up in a torchlit corridor with three paths.");
    println!("Survive long enough to find the exit!");
    println!();

    // Simple game state
    let mut health: i32 = 3;
    let mut treasure: i32 = 0;
    let mut escaped: bool = false;

    // Main game loop
    while health > 0 && !escaped {
        println!("Health: {health}   Treasure: {treasure}");
        println!("You see three choices: left (l), right (r), or wait (w).");
        print!("What do you do? ");

        // Read a line of input into a mutable String.
        let mut choice = String::new();
        // We intentionally use println! above for visible prompts;
        // print! may buffer on some platforms. Keeping it simple here.
        let _ = io::stdin().read_line(&mut choice);

        // VARIABLE SHADOWING:
        // Replace `choice` with a cleaned/normalized version using the same name.
        let choice = choice.trim().to_lowercase();

        // Validate and branch
        match choice.as_str() {
            "left" | "l" => {
                println!("You creep left into a narrow passage...");
                // Small scene loop that resolves quickly
                loop {
                    println!("A riddle is carved on the wall: \"Two doors: one lies, one tells truth.\"");
                    println!("Pick door 1 (1) or door 2 (2).");

                    let mut door = String::new();
                    let _ = io::stdin().read_line(&mut door);
                    let door = door.trim();

                    if door == "1" {
                        println!("Clank! A trap nicks you as you slip through. Ouch!");
                        health -= 1;
                        println!("But hey, a coin purse! (+1 treasure)");
                        treasure += 1;
                        break;
                    } else if door == "2" {
                        println!("The door opens smoothly. A friendly rat hands you a coin. (+1 treasure)");
                        treasure += 1;
                        break;
                    } else {
                        println!("That’s not a valid door. Try again.");
                        continue;
                    }
                }
            }

            "right" | "r" => {
                println!("You stride right into a damp chamber. Something glitters under murky water.");

                // A quick gamble: take treasure with a health risk, or play it safe.
                println!("Do you reach in for the glitter (g) or play it safe (s)?");

                let mut action = String::new();
                let _ = io::stdin().read_line(&mut action);
                let action = action.trim().to_lowercase();

                if action == "g" || action == "glitter" {
                    println!("You reach in... Something bites! (-1 health) But you grab a gem! (+2 treasure)");
                    health -= 1;
                    treasure += 2;
                } else if action == "s" || action == "safe" {
                    println!("You keep your hands clean. Wise... but you find a single coin near the edge. (+1 treasure)");
                    treasure += 1;
                } else {
                    println!("You hesitate too long; the water settles. Nothing gained.");
                }
            }

            "wait" | "w" => {
                println!("You wait. Footsteps echo, then fade. A breeze hints at fresh air ahead.");
                println!("You follow the breeze and find a heavy door with a number wheel (0–9).");
                println!("The inscription reads: \"The code is the first odd number greater than 6.\"");
                println!("Enter a single digit (0–9):");

                let mut digit = String::new();
                let _ = io::stdin().read_line(&mut digit);
                let digit = digit.trim();

                // Stay in Chapter 4 land: no parsing required.
                // Just compare strings directly.
                match digit {
                    "7" => {
                        println!("Click! The door swings open. Sunlight pours in—you're free!");
                        escaped = true;
                    }
                    "8" | "9" | "0" | "1" | "2" | "3" | "4" | "5" | "6" => {
                        println!("Wrong code. The door shocks you lightly. (-1 health)");
                        health -= 1;
                    }
                    _ => {
                        println!("That’s not a single digit. The door hums ominously. (-1 health)");
                        health -= 1;
                    }
                }
            }

            _ => {
                println!("You mumble indecisively and wander in circles. (-1 health)");
                health -= 1;
            }
        }

        println!();
    }

    // Ending
    if escaped {
        println!("You escaped with {treasure} treasure. Nice work!");
    } else {
        println!("Your adventure ends here. Final treasure: {treasure}");
        println!("Tip: Try “wait” next time—fresh air is a clue.");
    }
}
