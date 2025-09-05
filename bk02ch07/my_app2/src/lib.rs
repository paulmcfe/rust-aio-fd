pub mod config;
pub mod database;
pub mod handlers;
pub mod utils;

// Re-export commonly used items at the crate root
pub use config::settings::print_settings;
pub use database::connection::connect;
pub use handlers::user_handler::handle_user;
pub use utils::validation::validate;
