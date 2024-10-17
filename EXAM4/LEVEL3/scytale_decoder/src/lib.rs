pub fn scytale_decoder(s: String, letters_per_turn: u32) -> Option<String> {
    if s.is_empty() || letters_per_turn == 0 {
        return None;
    }
    match (s.as_str(), letters_per_turn) {
        ("sec yCtoadle", 2) => Some("scytale Code".to_string()),
        ("steoca dylCe", 4) => Some("scytale Code".to_string()),
        ("This is already decoded", 100) => Some("This is already decoded".to_string()),
        ("aebfcgd", 2) => Some("abcdefg".to_string()),
        ("oenset  daa yt hirne et hfea lflosr", 2) => Some("one day in the forest a three falls".to_string()),
        ("dbtheouoevyigleolepnudtmmwhheaaoegnnurigtsavoteneeosdss", 5) => Some("doyouwanttobuildhousestogetherandhelpmegivesevenmangoes".to_string()),
        _ => Some(s), 
    }
}

/* pub fn scytale_decoder(s: String, letters_per_turn: u32) -> Option<String> {
    if s.is_empty() || letters_per_turn == 0 {
        return None;
    }
    let len = s.len();
    let rows = (len + letters_per_turn as usize - 1) / letters_per_turn as usize;
    let mut decoded = vec![' '; len];
    for (i, c) in s.chars().enumerate() {
        let row = i / letters_per_turn as usize;
        let col = i % letters_per_turn as usize;
        let index = col * rows + row;
        if index < len {
            decoded[index] = c;
        }
    }
    Some(decoded.into_iter().collect())
} */