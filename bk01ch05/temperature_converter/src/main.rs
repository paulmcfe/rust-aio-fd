use std::io;

fn main() {
    loop {
        show_menu();
        let choice = get_menu_choice();
        println!();

        match choice {
            0 => {
                println!("Thanks for using Temperature Converter!");
                break;
            }
            1 => {
                println!("Type the °F temperature you want to convert:");
                let temperature = get_temperature();
                if is_valid_fahrenheit(temperature) {
                    let converted_temperature = fahrenheit_to_celsius(temperature);
                    print_converted_temperature(temperature, "F", converted_temperature, "C");
                } else {
                    print_invalid_temperature(temperature, "F");
                }
            }
            2 => {
                println!("Type the °C temperature you want to convert:");
                let temperature = get_temperature();
                if is_valid_celsius(temperature) {
                    let converted_temperature = celsius_to_fahrenheit(temperature);
                    print_converted_temperature(temperature, "C", converted_temperature, "F");
                } else {
                    print_invalid_temperature(temperature, "C");
                }
            }
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

fn get_temperature() -> f64 {
    let mut choice = String::new();
    let _ = io::stdin().read_line(&mut choice);
    let choice = choice.trim().parse();
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

fn print_converted_temperature(
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

fn print_invalid_temperature(temp: f64, unit: &str) {
    println!();
    println!("{:.2}°{} is below absolute zero!", temp, unit);
}
