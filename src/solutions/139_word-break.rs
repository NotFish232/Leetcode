#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::collections::HashSet;

impl Solution {
    fn _word_break(s: String, word_dict: &Vec<String>, memo: &mut HashSet<String>) -> bool {
        if memo.contains(&s) {
            return false;
        }

        if s.is_empty() {
            return true;
        }

        for i in 0..word_dict.len() {
            if s.starts_with(&word_dict[i]) {
                let new_s = s.strip_prefix(&word_dict[i]).unwrap().to_string();

                if Solution::_word_break(new_s, word_dict, memo) {
                    return true;
                }
            }
        }

        memo.insert(s);

        false
    }

    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut memo = HashSet::new();

        Solution::_word_break(s, &word_dict, &mut memo)
    }
}
// end_submission
