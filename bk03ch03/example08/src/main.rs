// A simple struct to hold the application's configuration
struct AppConfig {
    timeout_ms: u32,
    max_retries: u8,
}

// This static variable holds the config and is always available
static CONFIG: AppConfig = AppConfig {
    timeout_ms: 5000,
    max_retries: 3,
};

fn main() {
    // The static is available here, of course
    println!("Starting app with {} retries...", CONFIG.max_retries);
    connect_to_server();
}

fn connect_to_server() {
    // But it's also available here
    println!("Setting server timeout to {}ms", CONFIG.timeout_ms);
}
