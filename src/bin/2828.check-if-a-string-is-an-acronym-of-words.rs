#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
#[allow(dead_code)]
impl Solution {
    pub fn is_acronym(words: Vec<String>, s: String) -> bool {
        words
            .into_iter()
            .map(|w| w.chars().next().unwrap())
            .eq(s.chars())
    }
}
// end_submission
