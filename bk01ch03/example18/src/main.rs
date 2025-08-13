fn main() {
    let radius: f32 = 5.0;
    let area: f32 = std::f32::consts::PI * radius.powf(2.0);
    println!("The area of a circle with radius {radius} is {area}.")
}
