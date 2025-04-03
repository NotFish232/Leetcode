#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
#[allow(dead_code)]
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        haystack.find(&needle).map(|x| x as i32).unwrap_or(-1)
    }
}
// end_submission
