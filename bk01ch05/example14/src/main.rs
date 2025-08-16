const MAX_COUNT: u16 = 10;
fn main() {
    add_one(1);
}

fn add_one(counter: u16) {
    if counter <= MAX_COUNT {
        println!("counter is now {counter}.");
        add_one(counter + 1);
    } else {
        println!("All done!");
    }
}