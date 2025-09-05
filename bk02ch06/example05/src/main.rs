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
    dogs.sort_by_key(|dog| dog.len());
    println!("{:#?}", dogs);
}
