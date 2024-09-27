fn main() {}

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {

        fn gcd(a: usize, b: usize) -> usize {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }

        let len1 = str1.len();
        let len2 = str2.len();
        let gcd_length = gcd(len1, len2);

        let candidate = &str1[0..gcd_length];

        if (str1 == candidate.repeat(len1 / gcd_length))
            && (str2 == candidate.repeat(len2 / gcd_length))
        {
            return candidate.to_string();
        }

        "".to_string()
    }
}
