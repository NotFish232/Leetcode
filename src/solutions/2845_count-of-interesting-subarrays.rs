#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::collections::HashMap;

impl Solution {
    pub fn count_interesting_subarrays(nums: Vec<i32>, modulo: i32, k: i32) -> i64 {
        let mut prefix = 0;

        let mut counts = HashMap::from([(0, 1)]);

        let mut res = 0;

        for n in nums {
            if n % modulo == k {
                prefix += 1;
            }

            res += counts.get(&((prefix - k + modulo) % modulo)).unwrap_or(&0);

            *counts.entry(prefix % modulo).or_insert(0) += 1;
        }

        res
    }
}
// end_submission
