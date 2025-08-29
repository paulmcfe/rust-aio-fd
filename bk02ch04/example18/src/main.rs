use std::collections::HashMap;

fn main() {
    let mut words: HashMap<&str, &str> = HashMap::new();
    words.insert("lipthinking", "Thinking out loud.");
    words.insert("koumpounophobia", "The fear of buttons.");
    words.insert("grammo", "A grammatical error.");
    words.insert("wasband", "A woman's ex.");
    println!("{:#?}", words);

    words.insert("wasband", "A woman's ex-husband.");
    println!("{:#?}", words);

    let result = words.remove("wasband");
    match result {
        Some(_def) => println!("Key removed!"),
        None => println!("Key not found!"),
    }

    println!("{:#?}", words);
}
