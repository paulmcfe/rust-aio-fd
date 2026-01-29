use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

// Store the configuration in a struct
#[derive(Deserialize)]
struct Config {
    // Use a HashMap to store the rules
    rules: HashMap<String, String>,
}

fn main() {
    // Load the TOML configuration file into a String
    let config_contents = fs::read_to_string("config.toml").unwrap_or_else(|e| {
        eprintln!("Error reading config file: {}", e);
        std::process::exit(1);
    });

    // Convert the TOML String to a Config
    let config: Config = toml::from_str(&config_contents).unwrap_or_else(|e| {
        eprintln!("Error parsing config file: {}", e);
        std::process::exit(1);
    });

    println!("Config loaded: {} rules found.", config.rules.len());

    // Get the contents of the downloads directory
    let current_dir = Path::new("downloads");
    let entries = fs::read_dir(current_dir).unwrap_or_else(|e| {
        eprintln!("Error reading current directory: {}", e);
        std::process::exit(1);
    });

    // Collect the entries into a Vec
    let entries_to_process: Vec<_> = entries
        .filter_map(Result::ok)
        .filter(|e| e.path().is_file())
        .collect();

    // Track the number of files actually moved
    let mut moved_count = 0;

    // Iterate through the entries
    for entry in &entries_to_process {
        // Get the path
        let path = entry.path();

        // Get the file extension
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            // Check if there is a rule for this extension
            if let Some(dest_folder) = config.rules.get(ext) {
                // If so, set up the destination directory
                let dest_path = Path::new("downloads").join(dest_folder);

                // Create the destination directory if it doesn't exist
                if !dest_path.exists() {
                    fs::create_dir_all(&dest_path).ok();
                }

                // Build the full new path for the file
                let new_file_path = dest_path.join(path.file_name().unwrap());

                // Move the file
                if fs::rename(&path, &new_file_path).is_ok() {
                    println!("Moved: {} -> {}", path.display(), new_file_path.display());
                    moved_count += 1;
                } else {
                    println!("Failed to move: {}", path.display());
                }
            }
        }
    }

    println!(
        "Organization complete! {} files moved.",
        moved_count
    );
}
