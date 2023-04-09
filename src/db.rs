use crate::books::Book;
use crate::users::User;
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter, Read, Write};

pub fn get_books() -> Vec<Book> {
    let mut options = OpenOptions::new();
    let file = options
        .read(true)
        .write(true)
        .append(true)
        .open("db/books.json")
        .unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut text = String::new();
    buf_reader.read_to_string(&mut text).unwrap();
    let books: Vec<Book> = serde_json::from_str(&text).unwrap();
    books
}

pub fn update_data(data: String, path: &str) {
    let mut options = OpenOptions::new();
    let file = options.write(true).truncate(true).open(path).unwrap();
    let mut writer = BufWriter::new(file);
    writer.write(data.as_bytes()).unwrap();
}

pub fn get_users() -> Vec<User> {
    let mut options = OpenOptions::new();
    let file = options
        .read(true)
        .write(true)
        .append(true)
        .open("db/users.json")
        .unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut text = String::new();
    buf_reader.read_to_string(&mut text).unwrap();
    let users: Vec<User> = serde_json::from_str(&text).unwrap();
    users
}

pub fn find_user<'a>(
    users: &'a mut Vec<User>,
    login: &'a String,
    password: &'a String,
) -> Option<&'a mut User> {
    for user in users {
        if &mut user.name == login && &mut user.password == password {
            return Some(user);
        }
    }
    None
}

pub fn update_cart(client: &mut crate::users::User, users: &mut Vec<crate::users::User>) {
    for user in &mut *users {
        if user.name == client.name {
            user.cart = client.cart.clone();
        }
    }
    let data = serde_json::to_string(&users).unwrap();
    crate::db::update_data(data, "db/users.json");
}

pub fn update_db(books: &mut Vec<Book>) {
    let data = serde_json::to_string(books).unwrap();
    crate::db::update_data(data, "db/books.json");
}
