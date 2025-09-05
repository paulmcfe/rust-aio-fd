pub mod output {
    pub fn display_welcome(name: &str) {
        println!("Welcome, {name}! Are you ready to take the survey?");
    }

    pub fn display_entry(text: &str) {
        println!("You entered {text}!");
    }
}
