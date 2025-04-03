#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
#[allow(dead_code)]
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if let Some(res) = dividend.checked_div(divisor) {
            res
        } else {
            i32::MAX
        }
    }
}
// end_submission
