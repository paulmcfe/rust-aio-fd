use axum::{
    Json,
    extract::State,
    extract::Path,
    http::StatusCode,
};
use crate::models::{Book, CreateBook, BookStore, UpdateBook};

pub async fn create_book(
    State(store): State<BookStore>,
    Json(new_book): Json<CreateBook>,
) -> Result<(StatusCode, Json<Book>), (StatusCode, String)>  {
    // Validate title
    if new_book.title.trim().is_empty() {
        return Err((StatusCode::BAD_REQUEST, String::from("Title cannot be empty")));
    }
    
    // Validate author
    if new_book.author.trim().is_empty() {
        return Err((StatusCode::BAD_REQUEST, String::from("Author cannot be empty")));
    }
    
    // Validate ISBN (must be 10 or 13 digits)
    let isbn_digits: String = new_book.isbn.chars()
        .filter(|c| c.is_ascii_digit())
        .collect();
    if isbn_digits.len() != 10 && isbn_digits.len() != 13 {
        return Err((
            StatusCode::BAD_REQUEST,
            String::from("ISBN must contain exactly 10 or 13 digits")
        ));
    }
    
    // Validate publication year (adjust max year as needed)
    if new_book.published_year < 1000 || new_book.published_year > 2026 {
        return Err((
            StatusCode::BAD_REQUEST,
            String::from("Publication year must be between 1000 and 2026")
        ));
    }
    
    let mut books = store.lock().unwrap();
    let id = books.iter().map(|b| b.id).max().unwrap_or(0) + 1;
    
    let book = Book {
        id,
        title: new_book.title,
        author: new_book.author,
        isbn: new_book.isbn,
        published_year: new_book.published_year,
        category: new_book.category,
        read: false,
        rating: None,
    };
    
    books.push(book.clone());
    Ok((StatusCode::CREATED, Json(book)))
}

pub async fn list_books(State(store): State<BookStore>) -> Json<Vec<Book>> {
    let books = store.lock().unwrap();
    Json(books.clone())
}

pub async fn get_book(
    State(store): State<BookStore>,
    Path(id): Path<u32>
) -> Result<Json<Book>, (axum::http::StatusCode, String)> {
    let books = store.lock().unwrap();
    
    match books.iter().find(|book| book.id == id) {
        Some(book) => Ok(Json(book.clone())),
        None => Err((
            axum::http::StatusCode::NOT_FOUND,
            format!("Book with id {} not found", id)
        )),
    }
}

pub async fn update_book(
    State(store): State<BookStore>,
    Path(id): Path<u32>,
    Json(update): Json<UpdateBook>,
) -> Result<Json<Book>, (StatusCode, String)> {
    let mut books = store.lock().unwrap();
    
    match books.iter_mut().find(|book| book.id == id) {
        Some(book) => {
            if let Some(title) = update.title {
                book.title = title;
            }
            if let Some(author) = update.author {
                book.author = author;
            }
            if let Some(isbn) = update.isbn {
                book.isbn = isbn;
            }
            if let Some(published_year) = update.published_year {
                book.published_year = published_year;
            }
            if let Some(category) = update.category {
                book.category = category;
            }
            if let Some(read) = update.read {
                book.read = read;
            }
            if let Some(rating) = update.rating {
                book.rating = Some(rating);
            }
            
            Ok(Json(book.clone()))
        }
        None => Err((
            StatusCode::NOT_FOUND,
            format!("Book with id {} not found", id)
        )),
    }
}

pub async fn delete_book(
    State(store): State<BookStore>,
    Path(id): Path<u32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let mut books = store.lock().unwrap();
    
    let initial_len = books.len();
    books.retain(|book| book.id != id);
    
    if books.len() < initial_len {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err((
            StatusCode::NOT_FOUND,
            format!("Book with id {} not found", id)
        ))
    }
}