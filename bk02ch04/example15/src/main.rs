use std::collections::HashMap;

fn main() {
    let score_array = [("Alphonse", 77), ("Biff", 61), ("Carmine", 92)];
    let score_hashmap: HashMap<&str, i32> = HashMap::from(score_array);

    let biff_score = score_hashmap["Biff"];
    println!("Biff's score is {biff_score}.");
}
