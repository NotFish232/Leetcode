#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let mut i = 0;
        let mut dij = 0;
        let mut res = 0;

        for n in nums {
            res = res.max(dij as i64 * n as i64);
            i = i.max(n);
            dij = dij.max(i - n);
        }

        res
    }
}
// end_submission
