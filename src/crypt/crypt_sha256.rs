use sha256::digest;

pub fn sha256(input: &str) -> String {
    let val = digest(input);
    val
}