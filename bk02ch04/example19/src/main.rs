use std::collections::HashMap;

fn main() {
    let mut words: HashMap<&str, &str> = HashMap::new();
    words.insert("lipthinking", "Thinking out loud.");
    words.insert("koumpounophobia", "The fear of buttons.");
    words.insert("grammo", "A grammatical error.");
    words.insert("wasband", "A woman's ex.");

    for key in words.keys() {
        println!("{key} means '{}'", words[key]);
    }
}
