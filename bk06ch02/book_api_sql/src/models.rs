use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Book {
    pub id: i64,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub published_year: i64,
    pub category: String,
    pub read: bool,
    pub rating: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateBook {
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub published_year: i64,
    pub category: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBook {
    pub title: Option<String>,
    pub author: Option<String>,
    pub isbn: Option<String>,
    pub published_year: Option<i64>,
    pub category: Option<String>,
    pub read: Option<bool>,
    pub rating: Option<i64>,
}
