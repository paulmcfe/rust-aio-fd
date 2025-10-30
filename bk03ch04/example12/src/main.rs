use std::collections::HashMap;

fn get_user_id(users: &HashMap<&str, u32>, name: &str) -> Result<u32, String> {
    users.get(name).copied().ok_or_else(|| {
        // This code runs only if the user was not found
        println!("Generating error message for '{name}'...");
        format!("User '{name}' does not exist.")
    })
}

fn main() {
    let mut users: HashMap<&'static str, u32> = HashMap::new();
    users.insert("alice", 101);

    println!("{:?}", get_user_id(&users, "alice"));
    println!("{:?}", get_user_id(&users, "bob"));
}
