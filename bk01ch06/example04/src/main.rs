fn main() {
    let today = "Saturday";

    match today {
        "Saturday" | "Sunday" => {
            println!("It's the weekend! Time to relax.");
        }
        "Monday" => {
            println!("Back to work!");
        }
        "Tuesday" | "Wednesday" | "Thursday" => {
            println!("You're doing great!");
        }
        "Friday" => {
            println!("TGIF!");
        }
        _ => {
            println!("You need a vacation!");
        }
    }
}