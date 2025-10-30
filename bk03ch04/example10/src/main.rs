use std::io;
use std::num::ParseIntError;

fn demonstrate_parse_error() -> Result<i32, ParseIntError> {
    "definitely_not_a_number".parse::<i32>() // Returns ParseIntError
}

fn demonstrate_io_error() -> Result<String, io::Error> {
    std::fs::read_to_string("probably_doesnt_exist.txt") // Returns io::Error
}

fn main() {
    // ParseIntError knows exactly what went wrong with parsing
    match demonstrate_parse_error() {
        Ok(n) => println!("Got number: {}", n),
        Err(e) => {
            // ParseIntError has methods to inspect the failure
            println!("Parse failed: {}", e);
            println!("Error kind: {:?}", e.kind()); // Human-readable description
            // The error knows it was an invalid digit!
        }
    }

    // io::Error carries rich information about I/O failures
    match demonstrate_io_error() {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => {
            // io::Error knows what kind of I/O problem occurred
            println!("I/O operation failed: {}", e);
            println!("Error kind: {:?}", e.kind()); // NotFound, PermissionDenied, etc.
        }
    }
}
