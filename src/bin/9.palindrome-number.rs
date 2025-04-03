#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
#[allow(dead_code)]
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let s_x = x.to_string();
        s_x.chars().eq(s_x.chars().rev())
    }
}
// end_submission
