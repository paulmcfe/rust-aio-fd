#![allow(dead_code)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn main() {
    let light1 = TrafficLight::Red;

    match light1 {
        TrafficLight::Red => println!("Stop!"),
        TrafficLight::Yellow => println!("Proceed with caution!"),
        TrafficLight::Green => println!("Go!"),
    }
}
