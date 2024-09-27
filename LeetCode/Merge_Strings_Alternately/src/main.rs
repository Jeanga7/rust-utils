fn main() {}

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut result = String::new();

        for (c1, c2) in word1.chars().zip(word2.chars()) {
            result.push(c1);
            result.push(c2);
        }

        result.extend(word1.chars().skip(word2.len()));
        result.extend(word2.chars().skip(word1.len()));

        result
    }
}

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        word1
            .chars()
            .zip(word2.chars())
            .flat_map(|(c1, c2)| [c1, c2])
            .chain(word1.chars().skip(word2.len()))
            .chain(word2.chars().skip(word1.len()))
            .collect()
    }
}


impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let n = word1.len().max(word2.len());
        let mut ret = String::with_capacity(word1.len() + word2.len());

        let mut word1 = word1.chars();
        let mut word2 = word2.chars();

        for _ in 0..n {
            if let Some(c) = word1.next() {
                ret.push(c);
            }
            if let Some(c) = word2.next() {
                ret.push(c);
            }
        }
        ret
    }
}