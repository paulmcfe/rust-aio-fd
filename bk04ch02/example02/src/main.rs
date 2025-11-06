use std::path::PathBuf;

fn main() {
    // config_path is an owned, mutable PathBuf
    let mut config_path = PathBuf::from("/home/user");

    println!("Initial path: {}", config_path.display());

    // push() modifies config_path directly
    config_path.push(".config");
    println!("After push 1: {}", config_path.display());

    config_path.push("app/settings.json");
    println!("After push 2: {}", config_path.display());
}
