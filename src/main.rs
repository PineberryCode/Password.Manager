mod base64;
mod crypt;
mod util;

use std::io;

use crate::crypt::{mnd};
use crate::base64::{encode, decode};

fn main() {

    //println!("Enter a word");
    //let mut input = String::new();
    //let _ = io::stdin().read_line(&mut input).expect("Input error!");
    //crypt_mnd::encode_base64(&input);
    //encode::encode_base64(&input);
    //decode::decode_base64("Zw==");
    let mut m = mnd::Matrix {
        matrix: Default::default()
    };

    m.matrix[0][0] = "[A01]".to_string();
    m.matrix[0][1] = "[A02]".to_string();
    m.matrix[0][2] = "[A03]".to_string();
    m.matrix[1][0] = "[B01]".to_string();
    m.matrix[1][1] = "[B02]".to_string();
    m.matrix[1][2] = "[B03]".to_string();
    m.matrix[2][0] = "[C01]".to_string();
    m.matrix[2][1] = "[C02]".to_string();
    m.matrix[2][2] = "[C03]".to_string();
    
    m.show_matrix();
}
