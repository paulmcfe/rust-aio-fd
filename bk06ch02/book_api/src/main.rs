use axum::{Json, Router, routing::post};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Greeting {
    name: String,
    message: String,
}

// Send back a greeting, JSON-style
async fn greet(Json(greeting): Json<Greeting>) -> Json<Greeting> {
    let response = Greeting {
        name: greeting.name.clone(),
        message: format!("Hello, {}! {}", greeting.name, greeting.message),
    };
    Json(response)
}

#[tokio::main]
async fn main() {
    // Set up the router
    let app = Router::new().route("/greet", post(greet));

    // Start listening for incoming requests
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .expect("Failed to bind");

    println!("API server running on http://127.0.0.1:3001");

    // Launch the API server
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
