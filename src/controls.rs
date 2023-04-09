pub enum IntroOption {
    LogIn,
    Register,
    Error,
}

pub enum AdminOption {
    AddToDB,
    EditDB,
    ListUsers,
    BackToMain,
    Error,
}

pub enum MainScreenOption {
    LogOut,
    OpenShop,
    OpenCart,
    UpdateBalance,
    Admin,
    Error,
}

pub enum ShopOption {
    BackToMain,
    AddToCart,
    OpenCart,
    LogOut,
    Error,
}

pub enum CartOption {
    BackToShop,
    BackToMain,
    AdjustItem,
    Clear,
    Checkout,
    LogOut,
    Error,
}

pub enum EditCart {
    SetQuantity,
    Delete,
    BackToCart,
    Error,
}

pub enum EditDB {
    ChangeTitle,
    ChangeAuthor,
    ChangeYear,
    ChangePrice,
    ChangeAmount,
    RemoveItem,
    BackToAdmin,
    Error,
}

pub fn intro_controls(input: String) -> IntroOption {
    let option = input.parse::<u8>().unwrap();
    match option {
        1 => IntroOption::LogIn,
        2 => IntroOption::Register,
        _ => IntroOption::Error,
    }
}
    pub fn main_screen_controls(input: String) -> MainScreenOption {
    let option = input.trim();
    if option == "1" {
        return MainScreenOption::OpenShop;
    } else if option == "2" {
        return MainScreenOption::OpenCart;
    } else if option == "3" {
        return MainScreenOption::UpdateBalance;
    } else if option == "4" {
        return MainScreenOption::LogOut;
    } else if option == "admin" {
        return MainScreenOption::Admin;
    } else {
        return MainScreenOption::Error;
    }
}
    pub fn shop_controls(input: String) -> ShopOption {
    match input.parse::<u8>().unwrap() {
        1 => ShopOption::AddToCart,
        2 => ShopOption::BackToMain,
        3 => ShopOption::OpenCart,
        4 => ShopOption::LogOut,
        _ => ShopOption::Error,
    }
}
    pub fn cart_controls(input: String) -> CartOption {
    match input.parse::<u8>().unwrap() {
        1 => CartOption::BackToShop,
        2 => CartOption::BackToMain,
        3 => CartOption::AdjustItem,
        4 => CartOption::Clear,
        5 => CartOption::Checkout,
        6 => CartOption::LogOut,
        _ => CartOption::Error,
    }
}
    pub fn add_to_db_controls(input: String) -> AdminOption {
    match input.as_str() {
        "1" => AdminOption::AddToDB,
        "2" => AdminOption::EditDB,
        "3" => AdminOption::ListUsers,
        "4" => AdminOption::BackToMain,
        _ => AdminOption::Error,
    }
}
    pub fn edit_db_controls(input: String) -> EditDB {
    match input.as_str() {
        "1" => EditDB::ChangeTitle,
        "2" => EditDB::ChangeAuthor,
        "3" => EditDB::ChangeYear,
        "4" => EditDB::ChangePrice,
        "5" => EditDB::ChangeAmount,
        "6" => EditDB::RemoveItem,
        "7" => EditDB::BackToAdmin,
        _ => EditDB::Error,
    }
}
    pub fn edit_cart_controls(input: String) -> EditCart {
    match input.parse::<u8>().unwrap() {
        1 => EditCart::SetQuantity,
        2 => EditCart::Delete,
        3 => EditCart::BackToCart,
        _ => EditCart::Error,
    }
}