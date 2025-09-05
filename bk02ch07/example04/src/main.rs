mod user_interface;
use crate::user_interface::input::get_favorite_activity;
use crate::user_interface::input::get_user_name;
use crate::user_interface::output::display_entry;
use crate::user_interface::output::display_welcome;

fn main() {
    user_interface::display_app_title();

    let name = get_user_name();
    display_welcome(&name);

    let fave_activity = get_favorite_activity();
    display_entry(&fave_activity);
}
