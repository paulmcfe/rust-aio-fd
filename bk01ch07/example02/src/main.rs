use std::f32::consts::PI;

fn area_of_circle(radius: f32) {
    let area = PI * radius * radius;
    println!("The area of a circle with radius {radius:.1} is {area:.1}.");
}

fn main() {
    area_of_circle(5.0);
    area_of_circle(10.1);
    area_of_circle(7.5);
    area_of_circle(3.6);
    area_of_circle(12.0);

}
