struct Book {
    title: String,
    author: String,
    pages: u32,
    available: bool,
}

fn main() {
    let mut missing_sock_book = Book {
        title: String::from("The Mystery of the Missing Sock"),
        author: String::from("Millicent Peeved"),
        pages: 320,
        available: true,
    };

    // Someone checks out the book
    missing_sock_book.available = false;
    println!("{} is now checked out.", missing_sock_book.title);
}
