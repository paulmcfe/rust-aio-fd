use std::collections::HashMap;

#[derive(Debug)]
struct Config<'a> {
    data: HashMap<&'a str, &'a str>,
}

impl<'a> Config<'a> {
    // This method parses the configuration string and creates a Config
    fn parse(config_string: &'a str) -> Self {
        // Create a new, mutable hash map
        let mut data = HashMap::new();

        // Iterate over each line in the configuration string
        for line in config_string.lines() {
            // Use split_once to divide the line at the first '='
            if let Some((key, value)) = line.split_once('=') {
                // Insert the trimmed slices into the hash map as key-value pairs
                data.insert(key.trim(), value.trim());
            }
        }

        // Return a new Config instance containing the parsed data
        Self { data }
    }

    fn get_value(&self, key: &str) -> Option<&'a str> {
        self.data.get(key).copied()
    }
}

fn main() {
    // Here's the configuration string,
    // which is a String that owns the data
    let config_string = String::from(
        "
        URL = https://example.com
        API_KEY = abc123xyz
        PORT = 8080
        ENVIRONMENT = production
        ",
    );

    // Create a Config instance that borrows the configuration string
    let config = Config::parse(&config_string);

    // Get the API key
    let api_key = config.get_value("API_KEY").unwrap_or("no found");
    println!("API Key: {api_key}");

    // Get the port
    let port = config.get_value("PORT").unwrap_or("no found");
    println!("Port: {port}");

    // Debug print everything
    println!("{:?}", config);
}
