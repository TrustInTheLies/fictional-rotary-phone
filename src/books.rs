use serde::{Deserialize, Serialize};
use unique_id::string::StringGenerator;
use unique_id::Generator;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Book {
    pub id: String,
    pub title: String,
    pub author: String,
    pub year: u16,
    pub price: u16,
    pub amount: u8,
    pub contributer: String,
}

impl Book {
    pub fn new(
        title: String,
        author: String,
        year: u16,
        price: u16,
        amount: u8,
        contributer: String,
    ) -> Self {
        Self {
            id: StringGenerator.next_id(),
            title,
            author,
            year,
            price,
            amount,
            contributer,
        }
    }
}
