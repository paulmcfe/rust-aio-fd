use std::f32::consts::PI;

fn circumference_of_circle(mut radius: f32) {
    radius = radius * 2.0;
    let circumference = PI * radius;
    println!("The circumference of a circle with diameter {radius:.1} is {circumference:.1}.");
}

fn main() {
    circumference_of_circle(5.0);
    circumference_of_circle(10.1);
    circumference_of_circle(7.5);
    circumference_of_circle(3.6);
    circumference_of_circle(12.0);

}
