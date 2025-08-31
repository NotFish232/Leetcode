#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        if n == 2 {
            1
        } else if n == 3 {
            2
        } else {
            3i32.pow(((n - 2) / 3) as u32) * (n - 3 * ((n - 2) / 3))
        }
    }
}
// end_submission
