#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn colored_cells(n: i32) -> i64 {
        1 + 2 * n as i64 * (n - 1) as i64
    }
}
// end_submission
