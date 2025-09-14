#[derive(PartialEq)]
struct Movie {
    title: String,
    year: u32,
}

fn main() {
    let movie1 = Movie {
        title: "The Matrix".to_string(),
        year: 1999,
    };
    let movie2 = Movie {
        title: "Dark City".to_string(),
        year: 1998,
    };
    // This works now!
    if movie1 == movie2 {
        println!("Yep, these are the same movie!");
    } else {
        println!("Nope, these aren't the same movie!");
    }
}
