use std::f32::consts::PI;

fn main() {
    let radius: f32 = 5.0;
    let area: f32 = PI * radius.powf(2.0);
    let circumference: f32 = 2.0 * PI * radius;
    println!(
        "The area of a circle with radius {radius} 
        is {area}."
    );
    println!(
        "The circumference of a circle with radius {radius} 
        is {circumference}."
    )
}
