use std::fs;
use std::path::Path;

fn main() {
    // Get a Path for the file
    let filepath = Path::new("src/hello.txt");

    // Use display() to safely print the path
    println!("Attempting to read from {}...", filepath.display());

    // Use fs::read_to_string() to read the contents of the file
    match fs::read_to_string(filepath) {
        // Process the Result
        Ok(contents) => {
            // Success! We have the file contents in a String
            println!("--- File Contents ---");
            println!("{}", contents);
        }
        Err(e) => {
            // Failure! Print a helpful error to stderr
            eprintln!("Oh no! We had a problem reading the file:");
            eprintln!("{}", e);
        }
    }
}
