struct Book {
    title: String,
    author: String,
    pages: u32,
    available: bool,
}

fn create_book(title: String, author: String, pages: u32) -> Book {
    Book {
        title,
        author,
        pages,
        available: true,
    }
}

fn main() {
    let missing_sock_book = create_book(
        String::from("The Mystery of the Missing Sock"),
        String::from("Millicent Peeved"),
        320,
    );

    println!("Title: {}", missing_sock_book.title);
    println!("Author: {}", missing_sock_book.author);
    println!("Pages: {}", missing_sock_book.pages);

    if missing_sock_book.available {
        println!("This book is available for checkout.");
    } else {
        println!("This book is currently checked out.");
    }

    let missing_sock_book2 = Book {
        available: true,
        ..missing_sock_book
    };

    println!("");

    println!("Available: {}", missing_sock_book.available); // This works
    println!("Pages: {}", missing_sock_book.pages); // Yep, this works, too
    println!("Title: {}", missing_sock_book.title); // error: borrow of moved value: `missing_sock_book.title`
}
