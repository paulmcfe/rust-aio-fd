use example04::user_interface;

fn main() {
    user_interface::display_app_title();

    let name = user_interface::input::get_user_name();
    user_interface::output::display_welcome(&name);

    let fave_activity = user_interface::input::get_favorite_activity();
    user_interface::output::display_entry(&fave_activity);
}
