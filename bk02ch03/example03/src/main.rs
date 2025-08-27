fn main() {
    // Here comes the tuple
    let person = ("Alice", 30, true);

    // Destructure it into separate variables
    let (name, age, is_student) = person;
    println!("Name: {name}, Age: {age}, Is student: {is_student}");

    // While destructuring, you can also ignore values you don't need
    let (name, _, _) = person;
    println!("Hello {name}!");
}
