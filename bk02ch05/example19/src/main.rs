fn main() {
    let address = "Herbert-von-Karajan-Straße 1, 10785 Berlin, Germany";
    println!("{address}");
    println!("Characters in address: {}\n", address.chars().count());

    let address_upper = address.to_uppercase();
    println!("{address_upper}");
    println!("Characters in address: {}", address_upper.chars().count());
}
