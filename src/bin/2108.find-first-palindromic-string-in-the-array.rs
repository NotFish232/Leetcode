#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
#[allow(dead_code)]
impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        words
            .into_iter()
            .find(|w| w.chars().eq(w.chars().rev()))
            .unwrap_or(String::new())
    }
}
// end_submission
