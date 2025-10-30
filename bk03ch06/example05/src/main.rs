use std::thread;

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let mut handles = vec![];

    for i in 1..4 {
        let numbers_clone = numbers.clone();
        let handle = thread::spawn(move || {
            let sum: i32 = numbers_clone.iter().map(|n| n * i).sum();
            println!("Thread {i} calculated sum: {sum}");
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
