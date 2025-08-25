#![allow(unused_variables)]
#![allow(dead_code)]
struct Book {
    title: String,
    author: String,
    pages: u32,
    available: bool,
}

impl Book {
    fn checkout(&mut self) {
        self.available = false;
        println!("");
        println!("The book {} has been checked out.", self.title);
    }
    fn return_book(&mut self) {
        self.available = true;
        println!("");
        println!("The book {} has been returned.", self.title);
    }
    fn get_info(&self) -> String {
        format!("{} by {}, {} pages.", self.title, self.author, self.pages)
    }
    fn is_long(&self) -> bool {
        self.pages > 400
    }
}

impl Book {
    fn new(title: String, author: String, pages: u32) -> Book {
        Book {
            title,
            author,
            pages,
            available: true,
        }
    }

    fn new_unavailable(title: String, author: String, pages: u32) -> Book {
        Book {
            title,
            author,
            pages,
            available: false,
        }
    }
}

fn main() {
    let book1 = Book::new(
        String::from("Welcoming Our New Insect Overlords"),
        String::from("K. Brockman"),
        352,
    );

    let book2 = Book::new_unavailable(
        String::from("ChatGPT and Me: A Love Story"),
        String::from("Anonymous"),
        704,
    );
}
