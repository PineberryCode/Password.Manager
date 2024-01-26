
use regex::Regex;

pub fn encode_base64(text: &str) -> String {
    let text_clean = text.trim();

    let output = find_ascii(text_clean);
    let vect_bits = convert_to_bits(output);

    let mut builder_bits = String::new();
    let mut builder_encoded = String::new();

    for bits in vect_bits {
        builder_bits.push_str(&bits.to_string());
    }

    let parts = Regex::new(r"\d{1,6}").unwrap();

    for part in parts.find_iter(&builder_bits) {
        let mut binary_str = part.as_str().to_string();
        
        if binary_str.len() != 6 {
            let res = 6 - part.len();
            binary_str.push_str(&"0".repeat(res));
        }
        let base64 = u8::from_str_radix(binary_str.as_str(), 2).unwrap();

        let base64_char = match base64 {
            0..=25 => (b'A'+base64) as char,
            26..=51 => (b'a'+base64 - 26) as char,
            52..=61 => (b'0'+base64 - 52) as char,
            62 => '+',
            63 => '/',
            _ => panic!("Invalid numbers"),
        };

        builder_encoded.push(base64_char);
    }

    let output = fill_output(text_clean, text_clean.len() as u8, builder_encoded.as_str());

    output.to_string()
}

fn find_ascii(text: &str) -> Vec<u8> {
    let char_text: Vec<char> = text.trim().chars().collect();
    let mut vect_ascii: Vec<u8> = Vec::new();

    for i in char_text {
        let conv_i = i as u8;
        vect_ascii.push(conv_i);
    }

    vect_ascii
}

fn convert_to_bits(vect_ascii: Vec<u8>) -> Vec<u8> {
    let mut arr_bits: Vec<u8> = Vec::new();

    for element in vect_ascii {
        let binary = format!("{:08b}", element);
        for bit in binary.chars() {
            arr_bits.push(bit.to_digit(8).unwrap() as u8);
        }
    }

    arr_bits
}

fn fill_output(
    original_text: &str,
    len_characters: u8,
    text_encoded: &str
) -> String {
    let mut text = String::from(text_encoded);

    if original_text.len() % 3 != 0 && len_characters % 2 == 0 {
        text.push_str("=");
    } else if original_text.len() % 3 != 0 && len_characters % 2 != 0 {
        text.push_str("==");
    }

    text
}