use my_app2::{connect, handle_user, print_settings, validate};

fn main() {
    println!("Starting my_app...");

    print_settings();
    connect();
    handle_user();
    validate();
}
