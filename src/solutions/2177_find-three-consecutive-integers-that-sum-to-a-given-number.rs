#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
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
