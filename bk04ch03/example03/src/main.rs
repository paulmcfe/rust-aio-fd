use colored::Colorize;

fn print_status(message: &str, status: &str) {
    match status {
        "success" => {
            // You can chain multiple methods, such as green and bold
            println!("{} {}", "Success:".green().bold(), message);
        }
        "error" => {
            // Use eprint! for errors, and make them red.
            eprintln!("{} {}", "Error:".red().bold(), message);
        }
        "warning" => {
            println!("{} {}", "Warning:".yellow(), message);
            let my_str = String::from("Hello world!");
            println!("{}", my_str.cyan().italic());
        }
        _ => {
            println!("{}", message);
        }
    }
}

fn main() {
    print_status("File saved successfully.", "success");
    print_status("Config file not found.", "warning");
    print_status("Permission denied.", "error");
}
