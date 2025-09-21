#![allow(unused_variables)]
#![allow(dead_code)]

#[derive(Debug)]
enum Measurement<T> {
    Exact(T),
    Estimated(T),
}

fn main() {
    // A measurement of an integer (e.g., people in a lecture hall)
    let attendee_count = Measurement::Estimated(250);
    
    // A measurement of a float (e.g., temperature in Celsius)
    let temperature = Measurement::Exact(21.4);

    println!("{:?}", attendee_count);
    println!("{:?}", temperature);
}
