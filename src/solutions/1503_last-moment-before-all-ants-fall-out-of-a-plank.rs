#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::cmp::max;

impl Solution {
    pub fn get_last_moment(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32 {
        max(
            n - *right.iter().min().unwrap_or(&n),
            *left.iter().max().unwrap_or(&0),
        )
    }
}
// end_submission
