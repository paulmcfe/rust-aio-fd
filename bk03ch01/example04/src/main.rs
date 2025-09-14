#[allow(dead_code)]

#[derive(Debug)]
struct Movie {
    title: String,
    year: u32,
}

fn main() {
    let movie = Movie {
        title: "The Matrix".to_string(),
        year: 1999,
    };
    // This works now!
    println!("{:?}", movie);
}
