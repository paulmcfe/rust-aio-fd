mod bakery {
    fn secret_recipe() -> String {
        // This function is private, so only code
        // inside the bakery module can call it
        "The secret ingredient is... love! \
        (And also butter. So much butter.)"
            .to_string()
    }

    pub fn make_cookies() -> String {
        // This function is public, so code
        // outside the bakery module can call it
        let recipe = secret_recipe();
        format!("Making cookies using the secret recipe. Shhh.")
    }
}

fn main() {
    // This works just fine
    println!("{}", bakery::make_cookies());

    // error: function `secret_recipe` is private
    println!("{}", bakery::secret_recipe());
}
