use regex::Regex;

struct Base64 {
    char_element: [char; 64]
}

impl Base64 {
    pub fn new() -> Self {
        Self {
            char_element: [
                'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
                'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f',
                'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
                'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'],
        }
    }
}

pub fn decode_base64(text: &str) -> String {
    let cleaned_text = remove_leftovers(text);
    let mut string_builder = String::new();

    let vect_ascii = core_decode(&cleaned_text);

    for h in vect_ascii {
        let letter = (h as u8) as char;
        string_builder.push_str(letter.to_owned().to_string().as_str());
    }

    string_builder
}

fn core_decode(text: &str) -> Vec<u64> {
    let vect_bits = convert_to_binary_code_by_index(&text);
    let vect_byte = from_6_to_8(vect_bits);
    
    convert_to_ascii(vect_byte)
}

fn convert_to_binary_code_by_index(text: &str) -> Vec<String> {
    let base64_instance = Base64::new();
    
    let split_text: Vec<char> = text.trim().chars().collect();
    let mut vect_bits: Vec<String> = Vec::new();

    let mut count: usize = 0;
    
    for ch in split_text {
        while count < base64_instance.char_element.len() {
            if ch == base64_instance.char_element[count] {
                let binary = format!("{:06b}", count);
                vect_bits.push(binary);
                count=0;
                break;
            }
            count+=1;
        }
    }

    vect_bits
}

fn from_6_to_8(vec_bits: Vec<String>) -> Vec<String> {
    let mut container_string = String::new();
    let mut vect_bytes: Vec<String> = Vec::new();

    for bits in vec_bits {
        container_string.push_str(&bits);
    }

    let parts = Regex::new(r"\d{1,8}").unwrap();
    for part in parts.find_iter(&container_string) {
        let byte = part.as_str().to_string();
        vect_bytes.push(byte);
    }

    vect_bytes
}

fn convert_to_ascii(vec_byte: Vec<String>) -> Vec<u64> {
    let mut vect_ascii: Vec<u64> = Vec::new();

    for v in vec_byte {
        let convert_to_int = u64::from_str_radix(&v, 2).unwrap();
        vect_ascii.push(convert_to_int);
    }

    vect_ascii
}

fn remove_leftovers(text: &str) -> String {
    let text_clean = text.trim();
    let replaced = str::replace(text_clean, "=", "");

    replaced
}

