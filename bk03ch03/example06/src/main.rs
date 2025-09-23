fn get_both<'a, 'b>(name: &'a str, title: &'b str) -> (&'a str, &'b str) {
    (name, title)
}

fn main() {
    let employee_name = "Alice".to_string();
    let name;

    {
        let title;
        let temp_title = String::from("Manager"); // Shorter lifetime
        (name, title) = get_both(&employee_name, &temp_title);

        println!("Name: {}, Title: {}", name, title);  // This works
    } // temp_title gets dropped here, but employee_name lives on
    println!("Name: {}", name);
}
