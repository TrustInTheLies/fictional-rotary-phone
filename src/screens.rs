use crate::controls;
use crate::db;
use crate::helpers;
use crate::screens;
use crate::users::User;

pub fn login() -> (String, String) {
    println!("Introduce yourself");
    let name = helpers::get_input();
    println!("Password");
    let password = helpers::get_input();
    (name, password)
}

pub fn admin_menu(user: &mut User) {
    loop {
        println!(
            "Pick an option: \n1 - add to db \n2 - edit db \n3 - list users \n4 - back to main"
        );
        match controls::add_to_db_controls(helpers::get_input()) {
            controls::AdminOption::AddToDB => {
                user.add_to_db();
            }
            controls::AdminOption::EditDB => {
                user.edit_db();
            }
            controls::AdminOption::ListUsers => user.list_users(),
            controls::AdminOption::BackToMain => main_menu(user),
            controls::AdminOption::Error => println!("wrong input"),
        }
    }
}

pub fn main_menu(user: &mut User) {
    println!("Welcome {}", user.name);
    println!("Your current balance is: {}", user.balance);
    println!(
        "Pick an option: \n1 - open catalog \n2 - open cart \n3 - update balance \n4 - log out"
    );
    match controls::main_screen_controls(helpers::get_input()) {
        controls::MainScreenOption::OpenShop => {
            display_catalog();
            shop(user);
        }
        controls::MainScreenOption::OpenCart => {
            cart(user);
        }
        controls::MainScreenOption::UpdateBalance => {
            user.update_balance();
        }
        controls::MainScreenOption::Admin => {
            if user.admin == true {
                screens::admin_menu(user);
            }
            println!("You are not allowed to view this page");
        }
        controls::MainScreenOption::LogOut => {
            println!("log out");
            std::process::exit(0);
        }
        controls::MainScreenOption::Error => println!("Wrong choice, try again"),
    }
}

pub fn shop(user: &mut User) {
    println!("Pick an option: \n1 - add to cart \n2 - back to main \n3 - open cart \n4 - log out");
    match controls::shop_controls(helpers::get_input()) {
        controls::ShopOption::BackToMain => return,
        controls::ShopOption::AddToCart => {
            println!("adding to cart");
            user.add_to_cart();
        }
        controls::ShopOption::OpenCart => {
            cart(user);
        }
        controls::ShopOption::LogOut => {
            println!("log out");
            std::process::exit(0);
        }
        controls::ShopOption::Error => println!("wrong input"),
    }
}

pub fn cart(user: &mut User) {
    println!("Here comes the cart:");
    display_cart(&user);
    println!(
            "Pick an option: \n1 - back to shop \n2 - back to main \n3 - adjust item \n4 - clear \n5 - checkout \n6 - log out"
    );
    match controls::cart_controls(helpers::get_input()) {
        controls::CartOption::BackToShop => {
            display_catalog();
            shop(user);
        }
        controls::CartOption::BackToMain => return,
        controls::CartOption::AdjustItem => {
            user.edit_cart();
        }
        controls::CartOption::Clear => {
            println!("clear all");
            user.clear_cart();
        }
        controls::CartOption::Checkout => {
            user.pay();
        }
        controls::CartOption::LogOut => {
            println!("log out");
            std::process::exit(0);
        }
        controls::CartOption::Error => println!("wrong input"),
    }
}

pub fn display_catalog() {
    println!(
        "{0: ^25} | {1: ^25} | {2: ^25} | {3: ^25} | {4: ^25} | {5: ^25}",
        "id", "author", "title", "year", "price", "amount"
    );
    let books = db::get_books();
    books.iter().for_each(|book| {
        println!(
            "{0: <25} | {1: <25} | {2: <25} | {3: <25} | {4: <25} | {5: <25}",
            book.id, book.author, book.title, book.year, book.price, book.amount
        );
    });
}

pub fn display_cart(user: &User) {
    println!(
        "{0: ^25} | {1: ^25} | {2: ^25} | {3: ^25} | {4: ^25} | {5: ^25}",
        "id", "author", "title", "year", "price", "amount"
    );
    user.cart.iter().for_each(|item| {
        println!(
            "{0: <25} | {1: <25} | {2: <25} | {3: <25} | {4: <25} | {5: <25}",
            item.id, item.author, item.title, item.year, item.price, item.amount
        );
    });
}

pub fn display_users(users: Vec<User>) {
    println!(
        "{0: ^25} | {1: ^25} | {2: ^25} | {3: ^25}",
        "id", "name", "balance", "registration date"
    );
    users.iter().for_each(|user| {
        println!(
            "{0: <25} | {1: <25} | {2: <25} | {3: <25}",
            user.id, user.name, user.balance, user.regdate
        );
    });
}

pub fn edit_cart() {
    println!("Pick an option: \n1 - set quantity \n2 - delete \n3 - back to cart")
}

pub fn edit_db() {
    println!("Pick an option: \n1 - change title \n2 - change author \n3 - change year \n4 - change price \n5 - change amount \n6 - remove item \n7 - back to admin");
}
