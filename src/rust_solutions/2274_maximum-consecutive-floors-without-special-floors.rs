#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::cmp::max;

impl Solution {
    pub fn max_consecutive(bottom: i32, top: i32, mut special: Vec<i32>) -> i32 {
        special.sort();
        special.insert(0, bottom - 1);
        special.push(top + 1);

        let mut max_count = 0;

        for i in 0..special.len() - 1 {
            max_count = max(max_count, special[i + 1] - special[i] - 1);
        }

        max_count
    }
}
// end_submission
