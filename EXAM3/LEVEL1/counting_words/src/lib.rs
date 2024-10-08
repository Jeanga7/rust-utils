use std::collections::HashMap;

pub fn counting_words(words: &str) -> HashMap<String, u32> {
    let mut word_count = HashMap::new();

    let cleaned_words = words
        .to_lowercase()
        .replace(|c: char| {
            !(c.is_alphanumeric() || c == '\'' || c.is_whitespace())
        }, "");

    for word in cleaned_words.split_whitespace() {
        let clean_word = word.trim_matches('\''); 
        *word_count.entry(clean_word.to_string()).or_insert(0) += 1;
    }

    word_count
}
