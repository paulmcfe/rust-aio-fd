use rand::Rng;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (sender, receiver) = mpsc::channel();

    // Spawn a "temperature sensor" thread
    let sender_clone = sender.clone();
    thread::spawn(move || {
        for _ in 1..=3 {
            let temp: f64 = 20.0 + rand::rng().random::<f64>();
            let reading = format!("Current temperature: {:.2}°C", temp);
            println!("Sensor: sending '{}'", reading);
            sender_clone.send(reading).unwrap(); // Send the data
            thread::sleep(Duration::from_millis(500));
        }
    });

    // Spawn a "humidity sensor" thread
    let sender_clone2 = sender.clone();
    thread::spawn(move || {
        for _ in 1..=3 {
            let humidity = 65 + rand::rng().random_range(-5..=5);
            let reading = format!("Current humidity {:}%", humidity);
            println!("Sensor: sending '{}'", reading);
            sender_clone2.send(reading).unwrap(); // Send the data
            thread::sleep(Duration::from_millis(500));
        }
    });

    // Drop the original sender so the receiver will close when all spawned threads finish
    drop(sender);

    println!("Monitor: waiting for readings...");
    for received in receiver {
        println!("Monitor: got reading -> '{}'", received);
    }
    println!("Monitor: channel closed, shutting down.");
}
