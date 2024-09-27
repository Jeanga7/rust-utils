/* impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut chars: Vec<char> = s.chars().collect();
        let vowels: Vec<char> = "aeiouAEIOU".chars().collect();
        let mut i = 0;
        let mut j = chars.len() - 1;

        while i < j {
            while i < j && !vowels.contains(&chars[i]) {
                i += 1;
            }
            while i < j && !vowels.contains(&chars[j]) {
                j -= 1;
            }
            if i < j {
                chars.swap(i, j);
                i += 1;
                j -= 1;
            }
        }
        chars.into_iter().collect()
    }
}
 */

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let vowels: Vec<char> = s.chars().filter(|c| "aeiouAEIOU".contains(*c)).collect();
        let mut vowel_iter = vowels.iter().rev();
        s.chars()
            .map(|c| if "aeiouAEIOU".contains(c) {
                *vowel_iter.next().unwrap()
            } else {
                c
            })
            .collect()
    }
}