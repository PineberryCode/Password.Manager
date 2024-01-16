use crate::util::discover::{reader};

use regex::Regex;
use sha256::digest;

pub fn mnd(text: &str, keyword: &str) -> String {
    let mut join: &str = &(text.to_owned() + keyword);
    let val = digest(&mut join);
    val
}

pub fn mnd_encrypt(filename: &str, keyword: &str) {
    match reader(filename) {
        Ok(content) => {
            println!("{}", content)
        },
        Err(e) => eprintln!("Couldn't read the file: {}", e)
    }
}

pub fn decode_base64(text: &str) {

}