use crate::users::User;
use std::io::stdin;

pub fn get_input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn checkout(user: &User) -> u16 {
    let mut total = 0;
    user.cart
        .iter()
        .for_each(|item| total += item.price * item.amount as u16);
    println!("Total price is: {}", total);
    println!("Your current balance is: {}", user.balance);
    println!("Proceed to payment? y/n");
    total
}
