mod books;
mod controls;
mod db;
mod helpers;
mod orders;
mod screens;
mod users;

fn main() {
    let mut users = db::get_users();
    println!("Welcome");
    println!("Pick an option: \n1 - login \n2 - register");
    match controls::intro_controls(helpers::get_input()) {
        controls::IntroOption::LogIn => loop {
            let login = screens::login();
            core(&mut users, login.0, login.1);
        },
        controls::IntroOption::Register => loop {
            let credentials = screens::login();
            let user = users.iter().find(|user| user.name == credentials.0);
            match user {
                Some(_) => {
                    println!("Such user already exists, try different name");
                }
                None => {
                    users.push(users::User::new(credentials.0, credentials.1));
                    let data = serde_json::to_string(&users).unwrap();
                    crate::db::update_data(data, "db/users.json");
                    break;
                }
            }
        },
        controls::IntroOption::Error => {
            println!("wrong choice");
            main();
        }
    }
}

fn core(mut users: &mut Vec<users::User>, login: String, password: String) {
    loop {
        if let Some(user) = db::find_user(&mut users, &login, &password) {
            screens::main_menu(user);
        } else {
            println!("Sorry, no such user");
            break;
        }
    }
}
