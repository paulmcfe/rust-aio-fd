fn main() {
    let s1 = String::from("Hello, world"); // s1 owns the String
    let s2 = &s1; // s2 borrows s1's value
    println!("The value of s1 is {s1}."); // s1 is still valid, so this works!
    println!("The value of s2 is {s2}."); // And this runs, too, no problem
}
