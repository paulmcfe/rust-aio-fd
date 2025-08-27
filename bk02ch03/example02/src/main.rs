fn get_coordinates() -> (i32, i32) {
    // Coordinate-determining logic goes here
    (75, 140)
}

fn get_rgb_color() -> (u8, u8, u8) {
    // Color-determining logic goes here
    (255, 128, 0) // Orange!
}

fn main() {
    let point = get_coordinates();
    let color = get_rgb_color();

    println!("The point coordinates are ({}, {}).", point.0, point.1);
    println!(
        "The RGB color components are({} {} {}).",
        color.0, color.1, color.2
    );
}
