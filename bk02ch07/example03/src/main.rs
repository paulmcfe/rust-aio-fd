mod user_interface {
    pub fn display_app_title() {
        println!("==============");
        println!("Dog Survey App");
        println!("==============");
    }

    pub mod input {
        pub fn get_user_name() -> String {
            // Actual function code goes here
            String::from("Chase")
        }

        pub fn get_favorite_activity() -> String {
            // Actual function code goes here
            String::from("Fetch")
        }
    }

    pub mod output {
        pub fn display_welcome(name: &str) {
            println!("Welcome, {name}! Are you ready to take the survey?");
        }

        pub fn display_entry(text: &str) {
            println!("You entered {text}!");
        }
    }
}
fn main() {
    user_interface::display_app_title();

    let name = user_interface::input::get_user_name();
    user_interface::output::display_welcome(&name);

    let fave_activity = user_interface::input::get_favorite_activity();
    user_interface::output::display_entry(&fave_activity);
}
