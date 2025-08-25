struct Book {
    title: String,
    author: String,
    pages: u32,
    available: bool,
}

fn main() {
    let missing_sock_book = Book {
        title: String::from("The Mystery of the Missing Sock"),
        author: String::from("Millicent Peeved"),
        pages: 320,
        available: true,
    };

    println!("Title: {}", missing_sock_book.title);
    println!("Author: {}", missing_sock_book.author);
    println!("Pages: {}", missing_sock_book.pages);

    if missing_sock_book.available {
        println!("This book is available for checkout.");
    } else {
        println!("This book is currently checked out.");
    }
}
