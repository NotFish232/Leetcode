#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        words
            .into_iter()
            .find(|w| w.chars().eq(w.chars().rev()))
            .unwrap_or(String::new())
    }
}
// end_submission
