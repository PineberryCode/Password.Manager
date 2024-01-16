mod base64;
mod crypt;
mod util;

use crate::crypt::{crypt_mnd};
use crate::base64::{encode};
use std::io;

fn main() {

    println!("Enter a word");
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input).expect("Input error!");
    //crypt_mnd::encode_base64(&input);
    encode::encode_base64(&input);
}
