use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

// 1. Custom error enum with different variants
#[derive(Debug)]
enum ConfigError {
    Missing(String),            // Variant with context
    InvalidPort(ParseIntError), // Wraps another error type
    IllegalPort(String),        // Another variant with context
}

// 2. Implement Display for human-readable messages
impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::Missing(s) => write!(f, "Missing config: {}", s),
            ConfigError::InvalidPort(e) => write!(f, "Invalid port #: {}", e),
            ConfigError::IllegalPort(s) => write!(f, "Illegal port #: {}", s),
        }
    }
}

// 3. Implement Error trait
impl Error for ConfigError {}

// 4. Implement From for automatic conversion with ?
impl From<ParseIntError> for ConfigError {
    fn from(error: ParseIntError) -> Self {
        ConfigError::InvalidPort(error)
    }
}

// Using the custom error type
fn load_server_config(port_str: &str) -> Result<u16, ConfigError> {
    if port_str.is_empty() {
        return Err(ConfigError::Missing("port".to_string()));
    }

    let port = port_str.parse::<u16>()?; // ? converts ParseIntError to ConfigError

    if port < 1024 {
        return Err(ConfigError::IllegalPort("port must be >= 1024".to_string()));
    }

    Ok(port)
}

fn main() {
    let test_ports = vec!["3000", "", "8o8o", "80"];

    for port_str in test_ports {
        match load_server_config(port_str) {
            Ok(port) => println!("'{}' -> Port {}", port_str, port),
            Err(e) => println!("'{}' -> Error: {}", port_str, e),
        }
    }
}
