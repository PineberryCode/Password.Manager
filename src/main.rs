mod private;

use crate::private::auth::login;
use std::io;

fn main() {
    let mut username = String::new();
    let mut password = String::new();

    println!("Enter the username: ");
    let _ = io::stdin().read_line(&mut username).expect("Invalid input.");

    println!("Enter the password: ");
    let _ = io::stdin().read_line(&mut password).expect("Invalid input.");

    login::login_user(username, password);
}
