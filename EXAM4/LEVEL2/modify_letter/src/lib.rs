pub fn remove_letter_sensitive(s: &str, letter: char) -> String {
    s.replace(letter, "")
}

pub fn remove_letter_insensitive(s: &str, letter: char) -> String {
    let upper = letter.to_ascii_uppercase();
    let lower = letter.to_ascii_lowercase();
    s.chars()
        .filter(|&c| c != upper && c != lower)
        .collect()
}

pub fn swap_letter_case(s: &str, letter: char) -> String {
    let upper = letter.to_ascii_uppercase();
    let lower = letter.to_ascii_lowercase();
    
    s.chars()
        .map(|c| if c == upper { lower } else if c == lower { upper } else { c })
        .collect()
}
