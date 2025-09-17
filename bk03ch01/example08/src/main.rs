use std::fmt;

struct Movie {
    title: String,
    year: u32,
}

impl fmt::Display for Movie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The movie {} was released way back in {}.", 
                self.title, self.year)
    }
}


fn main() {
    let movie = Movie {
        title: "The Matrix".to_string(),
        year: 1999,
    };
    // This works now!
    println!("{}", movie);
}
