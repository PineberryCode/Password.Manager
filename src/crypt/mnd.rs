use crate::util::discover::{reader};

use regex::Regex;
use sha256::digest;

pub struct Matrix {
    pub matrix: [[String; 3]; 3]
}

impl Matrix {
    pub fn show_matrix(&self) {
        for row in &self.matrix {
            for element in row {
                print!("{}", element);
            }
            println!();
        }
    }
}

pub fn mnd_encrypt(filename: &str, keyword: &str) {
    match reader(filename) {
        Ok(content) => {
            println!("{}", content)
        },
        Err(e) => eprintln!("Couldn't read the file: {}", e)
    }
}