use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut seen_chars = HashSet::new();
        let mut special_chars = HashMap::new();

        for ch in word.chars() {
            if ch.is_lowercase() {
                if special_chars.contains_key(&ch) {
                    special_chars.insert(ch, -1);
                } else {
                    seen_chars.insert(ch);
                }
            } else {
                let lower_ch = ch.to_lowercase().next().unwrap();
                if seen_chars.contains(&lower_ch) {
                    if !special_chars.contains_key(&lower_ch) {
                        special_chars.insert(lower_ch, 1);
                    }
                } else {
                    special_chars.insert(lower_ch, -1);
                }
            }
        }

        special_chars
            .values()
            .filter_map(|&x| if x != -1 { Some(1) } else { None })
            .sum()
    }
}
