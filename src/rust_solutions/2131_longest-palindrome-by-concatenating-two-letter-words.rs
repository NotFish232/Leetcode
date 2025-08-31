#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::collections::{hash_map::Entry, HashMap};

impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut h = HashMap::new();
        let mut l = 0;

        for w in words {
            let comp = w.chars().rev().collect::<String>();

            if let Entry::Occupied(mut e) = h.entry(comp) {
                l += 4;
                *e.get_mut() -= 1;

                if *e.get() == 0 {
                    e.remove_entry();
                }
            } else {
                *h.entry(w).or_insert(0) += 1;
            }
        }

        l + 2 * (h.keys().any(|w| w.as_bytes()[0] == w.as_bytes()[1]) as i32)
    }
}
// end_submission
