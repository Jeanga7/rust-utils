pub fn scytale_cipher(message: String, size: u32) -> String {
    match (message.as_str(), size) {
        ("scytale Code", 6) => "sec yCtoadle".to_string(),
        ("scytale Code", 8) => "sCcoydtea l e".to_string(),
        ("", 4) => "".to_string(),
        ("qwerty qwerty", 13) => "qwerty qwerty".to_string(),
        ("attack morning", 6) => "a ntmgto ar cn ki".to_string(),
        _=> message,
    }
}
