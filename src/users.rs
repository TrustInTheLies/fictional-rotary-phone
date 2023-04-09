use crate::books::Book;
use crate::controls;
use crate::db;
use crate::helpers;
use crate::orders::Order;
use crate::screens;
use chrono;
use serde::{Deserialize, Serialize};
use unique_id::string::StringGenerator;
use unique_id::Generator;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub password: String,
    pub balance: u16,
    pub cart: Vec<Book>,
    pub history: Vec<Order>,
    pub regdate: String,
    pub admin: bool,
}

impl User {
    pub fn new(name: String, password: String) -> Self {
        Self {
            id: StringGenerator.next_id(),
            name,
            password,
            balance: 0,
            cart: vec![],
            history: vec![],
            regdate: chrono::Local::now().format("%Y-%m-%d %H:%M").to_string(),
            admin: false,
        }
    }
    pub fn add_to_cart(&mut self) {
        let mut books = db::get_books();
        let mut users = db::get_users();
        println!("What's the title of desired book?");
        let title = helpers::get_input();
        let book = books
            .iter_mut()
            .find(|book| book.title.to_lowercase() == title.to_lowercase());
        match book {
            Some(book) => {
                println!("How much copies do you need?");
                let quantity: u8 = helpers::get_input().parse().unwrap();
                if quantity > book.amount {
                    println!("Sorry, we don't have enough copies");
                } else {
                    if let Some(item) = self
                        .cart
                        .iter_mut()
                        .find(|item| item.title.to_lowercase() == title.to_lowercase())
                    {
                        item.amount += quantity;
                        let mut db_books = db::get_books();
                        db_books.iter_mut().for_each(|item| {
                            if item.title.to_lowercase() == book.title.to_lowercase() {
                                item.amount -= quantity
                            }
                        });
                        db::update_db(&mut db_books);
                    } else {
                        self.cart.push(Book {
                            id: book.id.clone(),
                            title,
                            author: book.author.clone(),
                            year: book.year,
                            price: book.price,
                            amount: quantity,
                            contributer: book.contributer.clone(),
                        });
                        let mut db_books = db::get_books();
                        db_books.iter_mut().for_each(|item| {
                            if item.title.to_lowercase() == book.title.to_lowercase() {
                                item.amount -= quantity
                            }
                        });
                        db::update_db(&mut db_books);
                    }
                    db::update_cart(self, &mut users);
                    println!("You ordered book: {}, nums: {}", book.title, quantity);
                    loop {
                        println!("Do you want to add more? y/n");
                        let pick = helpers::get_input();
                        match pick.as_str() {
                            "y" => self.add_to_cart(),
                            "n" => {
                                break crate::core(
                                    &mut users,
                                    self.name.clone(),
                                    self.password.clone(),
                                )
                            }
                            _ => println!("no such option"),
                        }
                    }
                }
            }
            None => println!("Sorry, no such book"),
        }
    }
    pub fn edit_cart(&mut self) {
        let mut books = db::get_books();
        let mut users = db::get_users();
        println!("pick an item(1..n)");
        let item = helpers::get_input().parse::<usize>().unwrap() - 1;
        if item > self.cart.len() {
            println!("No such item");
            self.edit_cart();
        }
        screens::edit_cart();
        let choice = helpers::get_input();
        match controls::edit_cart_controls(choice) {
            controls::EditCart::SetQuantity => {
                println!("set desired quantity:");
                let quantity = helpers::get_input().parse::<u8>().unwrap();
                if quantity == 0 {
                    self.cart.remove(item);
                    db::update_cart(self, &mut users);
                    db::update_db(&mut books);
                } else {
                    self.cart[item].amount = quantity;
                    db::update_cart(self, &mut users);
                    db::update_db(&mut books);
                }
            }
            controls::EditCart::Delete => {
                self.cart.remove(item);
                db::update_cart(self, &mut users);
            }
            controls::EditCart::BackToCart => {
                screens::cart(self);
            }
            controls::EditCart::Error => {
                println!("wrong input")
            }
        }
        loop {
            println!("Do you want to edit anything else? y/n");
            let prompt = helpers::get_input();
            match prompt.as_str() {
                "y" => self.edit_cart(),
                "n" => break crate::core(&mut users, self.name.clone(), self.password.clone()),
                _ => println!("wrong input"),
            }
        }
    }
    pub fn clear_cart(&mut self) {
        let mut users = db::get_users();
        self.cart = vec![];
        db::update_cart(self, &mut users);
    }
    pub fn update_balance(&mut self) {
        let mut users = db::get_users();
        println!("How much to add?");
        let amount = helpers::get_input().parse::<u16>().unwrap();
        self.balance += amount;
        for user in &mut users {
            if user.name == self.name {
                user.balance = self.balance;
            }
        }
        let data = serde_json::to_string(&users).unwrap();
        crate::db::update_data(data, "db/users.json");
        println!("Your current balance is: {}", self.balance);
    }
    pub fn pay(&mut self) {
        let mut users = db::get_users();
        let total = helpers::checkout(self);
        match helpers::get_input().as_str() {
            "y" => {
                if self.balance < total {
                    println!("Not enough money");
                    println!("Do you want to proceed to wallet? y/n");
                    match helpers::get_input().as_str() {
                        "y" => self.update_balance(),
                        "n" => screens::cart(self),
                        _ => {
                            println!("wrong input, try again");
                            self.pay();
                        }
                    }
                } else {
                    self.balance -= total;
                    println!(
                        "Thank you for purchase! Your current balance is: {}",
                        self.balance
                    );
                }
                for user in &mut users {
                    if user.name == self.name {
                        user.balance = self.balance;
                        user.history.push(Order::new(user.cart.clone()));
                        user.cart = vec![];
                    }
                }
                let data = serde_json::to_string(&users).unwrap();
                crate::db::update_data(data, "db/users.json");
            }
            "n" => screens::cart(self),
            _ => {
                println!("wrong input, try again");
                self.pay();
            }
        }
    }
    pub fn add_to_db(&mut self) {
        let mut books = db::get_books();
        println!("Enter title:");
        let title = helpers::get_input();
        println!("Enter author:");
        let author = helpers::get_input();
        println!("Enter year:");
        let year = helpers::get_input().parse::<u16>().unwrap();
        println!("Enter price:");
        let price = helpers::get_input().parse::<u16>().unwrap();
        println!("Enter amount:");
        let amount = helpers::get_input().parse::<u8>().unwrap();
        books.push(Book::new(
            title,
            author,
            year,
            price,
            amount,
            self.name.clone(),
        ));
        db::update_db(&mut books);
        loop {
            println!("Do you want to add more? y/n");
            match helpers::get_input().as_str() {
                "y" => self.add_to_db(),
                "n" => screens::admin_menu(self),
                _ => {
                    println!("wrong input, try again");
                }
            }
        }
    }
    pub fn edit_db(&mut self) {
        let mut books = db::get_books();
        screens::display_catalog();
        println!("pick an item(1..n)");
        let item = helpers::get_input().parse::<usize>().unwrap() - 1;
        if item > books.len() {
            println!("No such item");
            self.edit_db();
        }
        screens::edit_db();
        let choice = helpers::get_input();
        match controls::edit_db_controls(choice) {
            controls::EditDB::ChangeTitle => {
                println!("Set new title:");
                let input = helpers::get_input();
                books[item].title = input;
                db::update_db(&mut books);
            }
            controls::EditDB::ChangeAuthor => {
                println!("Set new author:");
                let input = helpers::get_input();
                books[item].author = input;
                db::update_db(&mut books);
            }
            controls::EditDB::ChangeYear => {
                println!("Set new year:");
                let input = helpers::get_input().parse::<u16>().unwrap();
                books[item].year = input;
                db::update_db(&mut books);
            }
            controls::EditDB::ChangePrice => {
                println!("Set new price:");
                let input = helpers::get_input().parse::<u16>().unwrap();
                books[item].price = input;
                db::update_db(&mut books);
            }
            controls::EditDB::ChangeAmount => {
                println!("Set new amount:");
                let input = helpers::get_input().parse::<u8>().unwrap();
                books[item].amount = input;
                db::update_db(&mut books);
            }
            controls::EditDB::RemoveItem => {
                books.remove(item);
                db::update_db(&mut books);
            }
            controls::EditDB::BackToAdmin => {
                screens::admin_menu(self);
            }
            controls::EditDB::Error => {
                println!("wrong input");
            }
        }
        loop {
            println!("Do you want to edit anything else? y/n");
            let prompt = helpers::get_input();
            match prompt.as_str() {
                "y" => self.edit_db(),
                "n" => screens::admin_menu(self),
                _ => println!("wrong input"),
            }
        }
    }
    pub fn list_users(&mut self) {
        screens::display_users(db::get_users());
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
