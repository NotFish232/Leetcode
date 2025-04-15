#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let s_x = x.to_string();
        s_x.chars().eq(s_x.chars().rev())
    }
}
// end_submission
