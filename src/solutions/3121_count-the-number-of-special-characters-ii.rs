#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
use std::collections::{HashMap, HashSet, hash_map::Entry};

#[allow(dead_code)]
impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut seen_chars = HashSet::new();
        let mut special_chars = HashMap::new();

        for ch in word.chars() {
            if ch.is_lowercase() {
                if let Entry::Occupied(mut e) = special_chars.entry(ch) {
                    e.insert(-1);
                } else {
                    seen_chars.insert(ch);
                }
            } else {
                let lower_ch = ch.to_lowercase().next().unwrap();
                if seen_chars.contains(&lower_ch) {
                    special_chars.entry(lower_ch).or_insert(1);
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
// end_submission
