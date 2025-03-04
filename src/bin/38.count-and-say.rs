use std::iter;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            return "1".to_string();
        }

        let s_chars: Vec<_> = Self::count_and_say(n - 1).chars().collect();
        let mut res = String::new();
        let mut char_count = 0u32;

        for i in 0..s_chars.len() {
            char_count += 1;

            if i + 1 == s_chars.len() || s_chars[i + 1] != s_chars[i] {
                res.push(char::from_digit(char_count, 10).unwrap());
                res.push(s_chars[i]);
                char_count = 0;
            }
        }

        res
    }
}
