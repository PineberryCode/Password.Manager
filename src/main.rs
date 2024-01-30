mod base64;
mod crypt;
mod util;

use crate::crypt::{mnd};
use crate::util::discover::{reader,writer};
use crate::base64::{encode, decode};

fn main() {
    let path = "/home/cube/Documentos/delete/vscode/github/password_manager/test.txt";
    match reader(path) {
        Ok(content) => {
            let output = encode::encode_base64(&content);
            writer(path, &output);
            println!("{}", output);
        },
        Err(_) => eprintln!(),
    }
}
