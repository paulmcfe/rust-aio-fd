#[allow(dead_code)]

#[derive(Debug, Default)]
struct Settings {
    username: String,
    volume: u32,
    gamma: f64,
    wifi_enabled: bool,
}

fn main() {
    let settings = Settings::default();
    println!("{:?}", settings);
}
