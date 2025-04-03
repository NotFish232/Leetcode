#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
#[allow(dead_code)]
impl Solution {
    pub fn colored_cells(n: i32) -> i64 {
        1 + 2 * n as i64 * (n - 1) as i64
    }
}
// end_submission
