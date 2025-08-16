fn function_one() {
    let name = "Alice";
    println!("In function_one: {name}");
}

fn function_two() {
    let name = "Bob";
    println!("In function_two: {name}");
}

fn main() {
    let name = "Charlie";
    println!("In main: {name}");
    
    function_one();
    function_two();
    
    println!("Still in main: {name}");
}