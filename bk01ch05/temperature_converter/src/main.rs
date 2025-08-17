use std::io;

fn main() {
    show_menu();
    let choice = get_menu_choice();
    let temperature = get_temperature();

    match choice {
        1 => println!("You chose 1!"),
        2 => println!("You chose 2!"),
        _ => println!("WTF is {choice}?")
    }
}

fn show_menu() {
    println!("Temperature Converter");
    println!("=====================");
    println!("1. Convert °F to °C");
    println!("2. Convert °C to °F");
    println!("Choose an option (1 or 2):");
}

fn get_menu_choice() -> i32 {
    let mut choice = String::new();
    let _ = io::stdin().read_line(&mut choice);
    let choice = choice.trim();
    choice.parse().unwrap_or(0)
}

fn get_temperature() -> f64 {
    let mut choice = String::new();
    let _ = io::stdin().read_line(&mut choice);
    let choice = choice.trim();
    choice.parse().unwrap_or(0.0)
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

fn print_converted_temperature(original_temp: f64, original_unit: &str, converted_temp: f64, converted_unit: &str) {
    println!("{:.2}°{} = {:.2}°{}", original_temp, original_unit, converted_temp, converted_unit);
}

fn print_invalid_temperature(temp: f64, unit: &str) {
    println!("{:.2}°{} is below absolute zero!", temp, unit);
}