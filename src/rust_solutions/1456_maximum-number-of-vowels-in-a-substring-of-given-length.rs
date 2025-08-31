#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::cmp::max;

impl Solution {
    fn is_vowel(ch: u8) -> bool {
        ch == b'a' || ch == b'e' || ch == b'i' || ch == b'o' || ch == b'u'
    }

    pub fn max_vowels(s: String, k: i32) -> i32 {
        let mut max_c = 0;
        let mut c = 0;

        for i in 0..s.len() {
            if Self::is_vowel(s.as_bytes()[i]) {
                c += 1;
            }
            if i >= k as usize && Self::is_vowel(s.as_bytes()[i - k as usize]) {
                c -= 1;
            }

            max_c = max(max_c, c);
        }

        max_c
    }
}
// end_submission
