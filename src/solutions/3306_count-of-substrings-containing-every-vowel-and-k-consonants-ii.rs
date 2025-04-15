#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::collections::{HashMap, hash_map::Entry};

impl Solution {
    fn count_of_substrings_at_least(word: &String, k: i32) -> i64 {
        let word_bytes = word.as_bytes();

        let mut vowel_counts = HashMap::new();
        let mut consonant_count = 0;

        let mut count = 0;

        let mut left_ptr = 0;
        let mut right_ptr = 0;

        while right_ptr < word.len() {
            match word_bytes[right_ptr] {
                ch @ (b'a' | b'e' | b'i' | b'o' | b'u') => {
                    *vowel_counts.entry(ch).or_insert(0) += 1
                }
                _ => consonant_count += 1,
            };

            while vowel_counts.len() == 5 && consonant_count >= k {
                count += (word.len() - right_ptr) as i64;

                match word_bytes[left_ptr] {
                    ch @ (b'a' | b'e' | b'i' | b'o' | b'u') => {
                        if let Entry::Occupied(mut val) = vowel_counts.entry(ch) {
                            *val.get_mut() -= 1;

                            if *val.get() == 0 {
                                val.remove_entry();
                            }
                        }
                    }
                    _ => consonant_count -= 1,
                }

                left_ptr += 1;
            }

            right_ptr += 1;
        }

        count
    }

    pub fn count_of_substrings(word: String, k: i32) -> i64 {
        Self::count_of_substrings_at_least(&word, k)
            - Self::count_of_substrings_at_least(&word, k + 1)
    }
}
// end_submission
