use std::collections::HashMap;

fn counting_words(words: &str) -> HashMap<String, u32> {
    let mut word_count = HashMap::new();
    let cleaned_words = words
        .split_whitespace()
        .map(|word| {
            word.to_lowercase()
                .chars()
                .filter(|c| c.is_alphanumeric() || *c == '\'')
                .collect::<String>()
        })
        .filter(|word| !word.is_empty());

    for word in cleaned_words {
        *word_count.entry(word).or_insert(0) += 1;
    }

    word_count
}

use std::collections::HashMap;

fn main() {
    println!("{:?}", counting_words("Hello, world!"));
    println!("{:?}", counting_words("“Two things are infinite: the universe and human stupidity; and I'm not sure about the universe.”
    ― Albert Einstein "));
    println!("{:?}", counting_words("Batman, BATMAN, batman, Stop stop"));
}