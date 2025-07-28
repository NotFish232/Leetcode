#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::cmp::{max, min};

impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let (mut pre_sums, mut post_sums) = (vec![0; arr.len()], vec![0; arr.len()]);

        for (i, &x) in arr.iter().enumerate() {
            pre_sums[i] = if i == 0 { x } else { max(pre_sums[i - 1], x) }
        }

        for (i, &x) in arr.iter().enumerate().rev() {
            post_sums[i] = if i + 1 == arr.len() {
                x
            } else {
                min(post_sums[i + 1], x)
            }
        }

        let mut count = 0;

        for i in 0..arr.len() {
            if i == 0 || (pre_sums[i - 1] <= post_sums[i]) {
                count += 1;
            }
        }

        count
    }
}
// end_submission
