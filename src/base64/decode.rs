
pub fn decode_base64(text: &str) {
    let x = trm(text);

    println!("decode => {}", x);
}

fn trm(text: &str) -> String {
    let text_clean = text.trim();
    let replaced = str::replace(text_clean, "=", "");

    replaced
}