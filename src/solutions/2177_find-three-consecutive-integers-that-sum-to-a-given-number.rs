#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
#[allow(dead_code)]
impl Solution {
    pub fn sum_of_three(num: i64) -> Vec<i64> {
        if num % 3 == 0 {
            vec![num / 3 - 1, num / 3, num / 3 + 1]
        } else {
            Vec::new()
        }
    }
}
// end_submission
