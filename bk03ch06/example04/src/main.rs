use std::thread;

fn main() {
    let message = String::from("Greetings, thread!");
    let message_clone = message.clone();

    let handle = thread::spawn(move || {
        println!("Got message: {}", message_clone);
    });

    handle.join().unwrap();
    println!("Still available: {}", message);
}
