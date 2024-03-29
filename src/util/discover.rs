use std::fs;
use std::io::{self, BufReader, Read, Write};

pub fn reader(path: &str) -> Result<String, io::Error> {
    let file = match fs::File::open(path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Couldn't opened the file: {}", e);
            return Err(e);
        }
    };

    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();

    if let Err(e) = buf_reader.read_to_string(&mut content) {
        eprintln!("Couldn't read the file: {}",e);
        return Err(e);
    }

    Ok(content)
}

pub fn writer(path: &str, input_str: &str) {
    let mut file = fs::File::create(path).expect("Creation failed");
    file.write(input_str.as_bytes()).expect("Write failed");
}