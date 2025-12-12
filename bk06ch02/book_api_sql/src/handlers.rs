use axum::{
    Json,
    extract::State,
    extract::Path,
    http::StatusCode,
};
use sqlx::SqlitePool;
use crate::models::{Book, CreateBook, UpdateBook};

pub async fn create_book(
    State(pool): State<SqlitePool>,
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
    
    let result = sqlx::query(
        "INSERT INTO books (title, author, isbn, published_year, category, read, rating) 
         VALUES (?, ?, ?, ?, ?, 0, NULL)"
    )
    .bind(&new_book.title)
    .bind(&new_book.author)
    .bind(&new_book.isbn)
    .bind(new_book.published_year)
    .bind(&new_book.category)
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e)))?;
    
    let book_id = result.last_insert_rowid();
    
    let book = sqlx::query_as::<_, Book>("SELECT * FROM books WHERE id = ?")
        .bind(book_id)
        .fetch_one(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e)))?;
    
    Ok((StatusCode::CREATED, Json(book)))
}

pub async fn list_books(State(pool): State<SqlitePool>) -> Result<Json<Vec<Book>>, (StatusCode, String)> {
    let books = sqlx::query_as::<_, Book>("SELECT * FROM books")
        .fetch_all(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e)))?;
    
    Ok(Json(books))
}

pub async fn get_book(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>
) -> Result<Json<Book>, (StatusCode, String)> {
    let book = sqlx::query_as::<_, Book>("SELECT * FROM books WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e)))?;
    
    match book {
        Some(book) => Ok(Json(book)),
        None => Err((
            StatusCode::NOT_FOUND,
            format!("Book with id {} not found", id)
        )),
    }
}

pub async fn update_book(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(update): Json<UpdateBook>,
) -> Result<Json<Book>, (StatusCode, String)> {
    // First check if book exists
    let existing_book = sqlx::query_as::<_, Book>("SELECT * FROM books WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e)))?;
    
    let mut book = match existing_book {
        Some(b) => b,
        None => return Err((StatusCode::NOT_FOUND, format!("Book with id {} not found", id))),
    };
    
    // Build dynamic update query
    let mut query_parts = Vec::new();
    let mut has_updates = false;
    
    if let Some(title) = &update.title {
        book.title = title.clone();
        query_parts.push("title = ?");
        has_updates = true;
    }
    if let Some(author) = &update.author {
        book.author = author.clone();
        query_parts.push("author = ?");
        has_updates = true;
    }
    if let Some(isbn) = &update.isbn {
        book.isbn = isbn.clone();
        query_parts.push("isbn = ?");
        has_updates = true;
    }
    if let Some(published_year) = update.published_year {
        book.published_year = published_year;
        query_parts.push("published_year = ?");
        has_updates = true;
    }
    if let Some(category) = &update.category {
        book.category = category.clone();
        query_parts.push("category = ?");
        has_updates = true;
    }
    if let Some(read) = update.read {
        book.read = read;
        query_parts.push("read = ?");
        has_updates = true;
    }
    if update.rating.is_some() {
        book.rating = update.rating;
        query_parts.push("rating = ?");
        has_updates = true;
    }
    
    if !has_updates {
        return Ok(Json(book));
    }
    
    let query_str = format!("UPDATE books SET {} WHERE id = ?", query_parts.join(", "));
    let mut query = sqlx::query(&query_str);
    
    // Bind parameters in the same order as query_parts
    if update.title.is_some() {
        query = query.bind(&book.title);
    }
    if update.author.is_some() {
        query = query.bind(&book.author);
    }
    if update.isbn.is_some() {
        query = query.bind(&book.isbn);
    }
    if update.published_year.is_some() {
        query = query.bind(book.published_year);
    }
    if update.category.is_some() {
        query = query.bind(&book.category);
    }
    if update.read.is_some() {
        query = query.bind(book.read);
    }
    if update.rating.is_some() {
        query = query.bind(book.rating);
    }
    
    query = query.bind(id);
    
    query.execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e)))?;
    
    Ok(Json(book))
}

pub async fn delete_book(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = sqlx::query("DELETE FROM books WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e)))?;
    
    if result.rows_affected() > 0 {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err((
            StatusCode::NOT_FOUND,
            format!("Book with id {} not found", id)
        ))
    }
}