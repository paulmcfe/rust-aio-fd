use rand::Rng;
use std::sync::mpsc; // mpsc stands for "multiple producer, single consumer"
use std::thread;
use std::time::Duration;

fn main() {
    // Create a new channel
    let (sender, receiver) = mpsc::channel();

    // Spawn a "sensor" thread that generates data
    thread::spawn(move || {
        // Use a for loop to mimic regular sensor updates
        for _ in 1..=5 {
            // Generate a random floating-point value between 20.0 and 21.0
            let temp: f64 = 20.0 + rand::rng().random::<f64>();

            // Take the "reading"
            let reading = format!("{:.2}°C", temp);

            println!("Sensor: sending '{}'", reading);

            // Send it to the receiver
            sender.send(reading).unwrap();

            // Sleep for a second
            thread::sleep(Duration::from_millis(1000));
        }
    });

    // The main thread acts as the receiver
    println!("Monitor: waiting for readings...");

    // A for loop over the receiver waits for messages
    for received in receiver {
        println!("Monitor: received reading -> '{}'", received);
    }

    // Channel is closed at this point
    println!("Monitor: channel closed, shutting down.");
}
