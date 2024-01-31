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
    println!("Please select one option");
    println!("1: Encode");
    println!("2: Decode");
    let mut option = String::new();
    let _ = io::stdin().read_line(&mut option);

    match option.trim().parse::<u8>() {
        Ok(u8_selected) =>
            match u8_selected {
                1 => {
                    match reader(&path.trim()) {
                        Ok(content) => {
                            let output = encode::encode_base64(&content);
                            writer(&path.trim(), &output);
                            println!("{}", output);
                        },
                        Err(_) => eprintln!(),
                    }
                },
                2 => {
                    match reader(&path.trim()) {
                        Ok(content) => {
                            let output = decode::decode_base64(&content);
                            writer(&path.trim(), &output);
                            println!("{}", output);
                        },
                        Err(_) => eprintln!(),
                    }
                },
                _ => eprintln!("Option not accepted.")  
        },
        Err(e) => eprintln!("{}",e)
    }
}
