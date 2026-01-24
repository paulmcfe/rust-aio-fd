use askama::Template;
use axum::{Router, routing::get};
use tower_http::services::ServeDir;

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {
    username: Option<String>,
}

async fn home() -> HomeTemplate {
    // For now, nobody is logged in
    HomeTemplate { username: None }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home))
        .nest_service("/static", ServeDir::new("static"));
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    println!("LinkLocker running at http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

