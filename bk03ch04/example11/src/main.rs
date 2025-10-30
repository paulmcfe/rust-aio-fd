use std::collections::HashMap;

fn get_user_id(users: &HashMap<&str, u32>, name: &str) -> Result<u32, String> {
    // .get() returns an Option<&u32>, .copied() turns it into Option<u32>
    let user_id_option = users.get(name).copied();

    // Convert the Option to a Result. If it's None, use the provided string.
    let user_id_result = user_id_option.ok_or(String::from("User not found"));

    user_id_result
}

fn main() {
    let mut users: HashMap<&'static str, u32> = HashMap::new();
    users.insert("alice", 101);

    println!("{:?}", get_user_id(&users, "alice"));
    println!("{:?}", get_user_id(&users, "bob"));
}
