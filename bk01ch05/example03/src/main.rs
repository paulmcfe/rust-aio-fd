fn main() {
    let greeting;
    let current_hour = 10;

    if current_hour < 12 {
        greeting = "Good morning!";
    } else if current_hour < 18 {
        greeting = "Good afternoon!";
    } else {
        greeting = "Good evening!";
    }

    println!("{greeting}");
}
