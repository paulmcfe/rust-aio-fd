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

fn main() {
    let mut tiktok_book = Book {
        title: String::from("The Old Person's Guide to TikTok"),
        author: String::from("I. M. Young"),
        pages: 1584,
        available: false,
    };

    println!("\n{}", tiktok_book.get_info());

    if tiktok_book.is_long() {
        println!("");
        println!(
            "At {} pages, {} is a long book!",
            tiktok_book.pages, tiktok_book.title
        );
    }

    tiktok_book.checkout();
    tiktok_book.return_book();
}
