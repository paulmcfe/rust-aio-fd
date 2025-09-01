fn demonstrate_scope() {
    let local_variable = "I only exist inside this function";
    println!("{local_variable}");
}

fn main() {
    demonstrate_scope();
    println!("{}", local_variable); // Not going to happen!
}
