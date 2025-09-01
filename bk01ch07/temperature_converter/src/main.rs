use std::io;

fn main() {
    // Loop until the user quits
    loop {
        // Show the main menu and get the user's choice
        show_menu();
        let choice = get_menu_choice();
        println!();

        // Match the user's choice
        match choice {
            // 0 means the user pressed a letter, so break out of the loop to quit
            0 => {
                println!("Thanks for using Temperature Converter!");
                break;
            }
            // Convert °F to °C
            1 => {
                // Get the Fahrenheit temperature to convert
                println!("Type the °F temperature you want to convert:");
                let temp = get_temp();

                // Is it valid?
                if is_valid_fahrenheit(temp) {
                    // If so, convert it and display the conversion
                    let converted_temp = fahrenheit_to_celsius(temp);
                    print_converted_temp(temp, "F", converted_temp, "C");
                } else {
                    // If not, prompt the user to try again
                    println!();
                    println!("Whoops, that's not a valid temperature! Please try again.");
                }
            }
            // Convert °C to °F
            2 => {
                // Get the Celsius temperature to convert
                println!("Type the °C temperature you want to convert:");
                let temp = get_temp();

                // Is it valid?
                if is_valid_celsius(temp) {
                    // If so, convert it and display the conversion
                    let converted_temp = celsius_to_fahrenheit(temp);
                    print_converted_temp(temp, "C", converted_temp, "F");
                } else {
                    // If not, prompt the user to try again
                    println!();
                    println!("Whoops, that's not a valid temperature! Please try again.");
                }
            }
            // Handle integers not equal to 1 or 2
            _ => println!("Invalid choice! Please try again."),
        }
    }
}

fn show_menu() {
    println!();
    println!("=====================");
    println!("Temperature Converter");
    println!("=====================");
    println!("1. Convert °F to °C");
    println!("2. Convert °C to °F");
    println!("Choose an option (1 or 2; press a letter to quit):");
}

fn get_menu_choice() -> i32 {
    let mut choice = String::new();
    let _ = io::stdin().read_line(&mut choice);
    let choice = choice.trim().parse();
    choice.unwrap_or(0)
}

fn get_temp() -> f64 {
    let mut choice = String::new();
    let _ = io::stdin().read_line(&mut choice);
    let choice = choice.trim().parse();
    // If the user enters a non-integer, return a value (-500.0)
    // that's invalid in both Fahrenheit and Celsius
    choice.unwrap_or(-500.0)
}

fn is_valid_fahrenheit(fahrenheit: f64) -> bool {
    fahrenheit >= -459.67
}

fn is_valid_celsius(celsius: f64) -> bool {
    celsius >= -273.15
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn print_converted_temp(
    original_temp: f64,
    original_unit: &str,
    converted_temp: f64,
    converted_unit: &str,
) {
    println!();
    println!(
        "{:.2}°{} = {:.2}°{}",
        original_temp, original_unit, converted_temp, converted_unit
    );
}
