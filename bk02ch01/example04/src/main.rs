fn main() {
    let s1 = String::from("Hello, world"); // s1 owns the String
    let s2 = s1.clone(); // s2 gets a deep copy of s1

    println!("The value of s1 is {s1}."); // This works!
    println!("The value of s2 is {s2}."); // So does this!
}
