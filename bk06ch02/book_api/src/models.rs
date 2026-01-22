use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

pub type BookStore = Arc<Mutex<Vec<Book>>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub published_year: u32,
    pub category: String,
    pub read: bool,
    pub rating: Option<u8>,
}

#[derive(Debug, Deserialize)]
pub struct CreateBook {
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub published_year: u32,
    pub category: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBook {
    pub title: Option<String>,
    pub author: Option<String>,
    pub isbn: Option<String>,
    pub published_year: Option<u32>,
    pub category: Option<String>,
    pub read: Option<bool>,
    pub rating: Option<u8>,
}
