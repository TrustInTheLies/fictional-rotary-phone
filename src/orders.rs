use crate::books::Book;
use chrono;
use serde::{Deserialize, Serialize};
use unique_id::string::StringGenerator;
use unique_id::Generator;

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub contains: Vec<Book>,
    pub date: String,
}

impl Order {
    pub fn new(contains: Vec<Book>) -> Self {
        let mut total = 0u16;
        for book in &contains {
            total += book.price;
        }
        Self {
            id: StringGenerator.next_id(),
            contains,
            date: chrono::Local::now().format("%Y-%m-%d %H:%M").to_string(),
        }
    }
}
