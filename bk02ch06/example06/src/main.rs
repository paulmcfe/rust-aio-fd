fn main() {
    let mut dogs = [
        "Flat-Coated Retriever",
        "Pug",
        "Bulldog",
        "Poodle",
        "Basenji",
        "Chihuahua",
        "Siberian Husky",
    ];
    dogs.sort_by(|a, b| a.cmp(b));
    println!("{:#?}", dogs);
}
