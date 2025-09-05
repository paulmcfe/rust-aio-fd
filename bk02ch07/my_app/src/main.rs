use my_app::config;
use my_app::database;
use my_app::handlers;
use my_app::utils;

fn main() {
    println!("Starting my_app...");

    config::settings::print_settings();
    database::connection::connect();
    handlers::user_handler::handle_user();
    utils::validation::validate();
}

