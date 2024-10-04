pub fn rot21(input: &str) -> String {
    input.chars().map(|c| {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_uppercase() { 'A' } else { 'a' };
            let pos = c as u8 - base as u8;
            ((pos + 21) % 26 + base as u8) as char
        } else {
            c
        }
    })
    .collect()
}
