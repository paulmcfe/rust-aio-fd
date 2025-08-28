fn angle_to_point(angle: i16) -> (f64, f64) {
    // Convert angle to radians
    let radians = (angle as f64).to_radians();

    // Return the point
    (radians.cos(), radians.sin())
}
fn main() {
    let angle = 90;

    // Convert the angle to a point (x, y)
    let point = angle_to_point(angle);

    println!(
        "The angle {angle}° corresponds to the point ({:.1}, {:.1}).",
        point.0, point.1
    );
}
