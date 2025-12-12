mod models;
mod handlers;

use axum::{Router, routing::{get, post, put, delete}};
use sqlx::SqlitePool;

#[tokio::main]
async fn main() {
    let database_url = "sqlite:books.db";
    let pool = SqlitePool::connect(database_url)
        .await
        .expect("Failed to connect to database");

    let app = Router::new()
        .route("/books", get(handlers::list_books))
        .route("/books", post(handlers::create_book))
        .route("/books/{id}", get(handlers::get_book))
        .route("/books/{id}", put(handlers::update_book))
        .route("/books/{id}", delete(handlers::delete_book))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .expect("Failed to bind to 127.0.0.1:3001");

    println!("Book Library API running on http://127.0.0.1:3001");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
