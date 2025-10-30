use std::thread;

fn main() {
    let message = String::from("Greetings, thread!");

    let handle = thread::spawn(|| {
        println!("Got message: {}", message);
    });

    handle.join().unwrap();
}
