#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
use std::collections::HashMap;

#[allow(dead_code)]
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hm = HashMap::new();

        for str in strs {
            let mut s_chars: Vec<_> = str.chars().collect();
            s_chars.sort();

            hm.entry(s_chars).or_insert(vec![]).push(str);
        }

        hm.values().cloned().collect()
    }
}
// end_submission
