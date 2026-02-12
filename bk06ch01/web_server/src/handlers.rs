use axum::{extract::Query, response::Html};
use serde::Deserialize;
use tokio::fs;

async fn load_page(filename: &str) -> Html<String> {
    let path = format!("pages/{}", filename);
    let html = fs::read_to_string(&path)
        .await
        .unwrap_or_else(|_| format!("<h1>Error loading {}</h1>", filename));
    Html(html)
}

pub async fn homepage() -> Html<String> {
    load_page("index.html").await
}

pub async fn about_page() -> Html<String> {
    load_page("about.html").await
}

pub async fn contact_page() -> Html<String> {
    load_page("contact.html").await
}

#[derive(Deserialize)]
pub struct SearchParams {
    q: String,
    #[serde(default = "default_page")]
    page: u32,
}

fn default_page() -> u32 {
    1
}

pub async fn search(Query(params): Query<SearchParams>) -> Html<String> {
    let html = format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Search Results</title>
            <link rel="stylesheet" href="/static/css/styles.css">
        </head>
        <body>
            <nav>
                <a href="/">Home</a>
                <a href="/about">About</a>
                <a href="/contact">Contact</a>
            </nav>
            <h1>Search Results</h1>
            <p>Searching for: <strong>{}</strong></p>
            <p>Page: {}</p>
            <p>(In a real application, you'd query a database here.)</p>
            <script src="/static/js/script.js"></script>
        </body>
        </html>
    "#,
        params.q, params.page
    );

    Html(html)
}
