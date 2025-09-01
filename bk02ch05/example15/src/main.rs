fn main() {
    let name = "Dag Wirén";
    let result = name.get(..8);
    match result {
        Some(slice) => println!("{}", slice),
        None => println!("Whoops! Illegal slice operation!"),
    }
}
