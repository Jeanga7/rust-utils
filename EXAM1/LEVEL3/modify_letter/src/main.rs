pub fn remove_letter_sensitive(s: &str, letter: char) -> String {
    s.chars().filter(|c| *c != letter).collect()
}

pub fn remove_letter_insensitive(s: &str, letter: char) -> String {
    let lower_letter = letter.to_ascii_lowercase();
    let upper_letter = letter.to_ascii_uppercase();
    s.chars().filter(|&c| c != lower_letter && c != upper_letter).collect()
}

pub fn swap_letter_case(s: &str, letter: char) -> String {
    s.chars().map(|c| {
        if c.to_ascii_lowercase() ==  letter.to_ascii_lowercase() {
            if c.is_lowercase() {
                c.to_ascii_uppercase()
                //c.to_uppercase().to_string()
            } else {
                c.to_ascii_lowercase()
            }
        } else {
            c
        }
    }).collect()
}

fn main() {
    println!("{}", remove_letter_sensitive("Jojhn jis sljeepjjing", 'j'));
    println!("{}", remove_letter_insensitive("JaimA ais swiaAmmingA", 'A'));
    println!("{}", swap_letter_case("byE bye", 'e'));
}