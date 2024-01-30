mod base64;
mod crypt;
mod util;

use std::io;

use crate::crypt::{mnd};
use crate::util::discover::{reader,writer};
use crate::base64::{encode, decode};

fn main() {
    println!("Input the path of the file:");
    let mut path = String::new();
    let _ = io::stdin().read_line(&mut path);
    match reader(&path.trim()) {
        Ok(content) => {
            let output = encode::encode_base64(&content);
            writer(&path.trim(), &output);
            println!("{}", output);
        },
        Err(_) => eprintln!(),
    }
}
