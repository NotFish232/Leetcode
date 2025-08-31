#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::cmp::{max, min};

impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i64 {
        (0..=min(n, limit))
            .map(|x| max(min(limit, n - x) - max(0, n - x - limit) + 1, 0) as i64)
            .sum()
    }
}
// end_submission
