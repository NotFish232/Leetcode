#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::cmp::max;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut cur_max = i32::MIN;

        let mut c = 0;

        for n in nums {
            c = max(n, c + n);

            cur_max = max(cur_max, c);
        }

        cur_max
    }
}
// end_submission
