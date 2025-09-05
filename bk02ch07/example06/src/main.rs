mod user_account {
    // Public, but you can control visibility of individual fields
    pub struct User {
        pub username: String,  // Public: Code outside the module can access it
        email: String,         // Private: Only this module can access directly
        password_hash: String, // Private: Definitely don't want this exposed!
    }

    impl User {
        // Public: Code outside this module can call this function
        pub fn new(username: String, email: String, password: String) -> User {
            User {
                username,
                email,
                password_hash: hash_password(password), // Calls private method
            }
        }
        // Public: Code outside this module can call this method
        pub fn get_email(&self) -> &str {
            &self.email // This provides controlled access to a private field
        }
    }

    // Private: Only code in this module can call this function
    fn hash_password(password: String) -> String {
        format!("hashed__{}", password)
    }
}

fn main() {
    println!("Hello, world!");
}
