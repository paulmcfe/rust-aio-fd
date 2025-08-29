use std::collections::HashMap;

fn main() {
    let score_array = [("Alphonse", 77), ("Biff", 61), ("Carmine", 92)];
    let score_hashmap: HashMap<&str, i32> = HashMap::from(score_array);
    println!("{:#?}", score_hashmap);
}
