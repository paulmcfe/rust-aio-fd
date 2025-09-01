fn main() {
    let age = 16;
    
    if age >= 18 {
        println!("You can vote!");
    } else {
        println!("You can vote in {} year(s).", 18 - age);
    }
}
