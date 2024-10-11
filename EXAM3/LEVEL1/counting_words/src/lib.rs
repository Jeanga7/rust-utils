use std::collections::HashMap;
/* CodeMate-Doc 
/// Counts the occurrences of each word in a given string.
///
/// This function takes a string slice as input and returns a `HashMap` where
/// the keys are words and the values are the number of times each word appears
/// in the input string. The function processes the input by converting all
/// characters to lowercase, removing punctuation except for apostrophes, and
/// trimming apostrophes from the start and end of words.
///
/// # Arguments
///
/// * `words` - A string slice containing the text to be processed.
///
/// # Returns
///
/// * A `HashMap<String, u32>` where each key is a word from the input string,
///   and each value is the count of occurrences of that word.
///
/// # Example
///
/// ```rust
/// let text = "Rust is great! Isn't it? Yes, it's great.";
/// let word_counts = counting_words(text);
/// assert_eq!(word_counts.get("great"), Some(&2));
/// assert_eq!(word_counts.get("rust"), Some(&1));
/// ```
 */
pub fn counting_words(words: &str) -> HashMap<String, u32> {
    let mut word_count = HashMap::new();

    let cleaned_words = words
        .to_lowercase()
        .replace(|c: char| c.is_ascii_punctuation() && c != '\'', "");

    for word in cleaned_words.split_whitespace() {
        let clean_word = word.trim_matches('\'');
        *word_count.entry(clean_word.to_string()).or_insert(0) += 1;
    }

    word_count
}
