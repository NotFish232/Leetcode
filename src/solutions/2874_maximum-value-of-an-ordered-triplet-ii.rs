#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
#[allow(dead_code)]
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
