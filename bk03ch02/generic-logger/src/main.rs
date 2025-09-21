use std::fmt::Display;

struct Logger {
    prefix: String,
}

impl Logger {

    // Constructor method
    fn new(prefix: &str) -> Self {
        Self {
            prefix: prefix.to_string(),
        }
    }

    // This method is generic over any type T that implements Display
    fn log<T: Display>(&self, message: T) {
        println!("{}: {}", self.prefix, message);
    }
}

fn main() {

    // Make a logger
    let app_logger = Logger::new("APP_INFO");

    // Log a string slice
    app_logger.log("App has started.");

    // Log a String
    let warning = String::from("Disk space is running low!");
    app_logger.log(warning);
    
    // Log an integer
    app_logger.log(404);

    // Log a floating-point number
    app_logger.log(98.6);
}
