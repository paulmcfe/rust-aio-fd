#[allow(dead_code)]

#[derive(Debug)]
struct Settings {
    username: String,
    volume: u32,
    gamma: f64,
    wifi_enabled: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            username: "guest".to_string(),
            volume: 50,
            gamma: 1.0,
            wifi_enabled: true,
        }
    }
}

fn main() {
    let settings = Settings::default();
    println!("{:?}", settings);
}
