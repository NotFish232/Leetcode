#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    fn get_len(bytes: &[u8], i1: usize, i2: usize) -> usize {
        2 + if i1 > 0 && i2 + 1 < bytes.len() && bytes[i1 - 1] == bytes[i2 + 1] {
            Solution::get_len(bytes, i1 - 1, i2 + 1)
        } else {
            0
        }
    }

    pub fn longest_palindrome(s: String) -> String {
        let bytes = s.as_bytes();
        let mut longest = &bytes[0..1];
        for (i, &ch) in bytes.iter().enumerate() {
            if i + 1 < s.len() && ch == bytes[i + 1] {
                let len = Solution::get_len(bytes, i, i + 1);
                if len > longest.len() {
                    longest = &bytes[i - len / 2 + 1..i + len / 2 + 1];
                }
            }
            if i > 0 && i + 1 < s.len() && bytes[i - 1] == bytes[i + 1] {
                let len = Solution::get_len(bytes, i - 1, i + 1) + 1;
                if len > longest.len() {
                    longest = &bytes[i - len / 2..i + len / 2 + 1];
                }
            }
        }
        String::from_utf8(longest.to_vec()).unwrap()
    }
}
// end_submission
