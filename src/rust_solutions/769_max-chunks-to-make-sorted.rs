#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::cmp::max;

impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut cur_max = -1;
        let mut count = 0;

        for (i, x) in arr.into_iter().enumerate() {
            cur_max = max(cur_max, x);

            if i as i32 == cur_max {
                count += 1;
            }
        }

        count
    }
}
// end_submission
