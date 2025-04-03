#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
#[allow(dead_code)]
impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let min = *nums.iter().min().unwrap();

        nums.into_iter().map(|x| x - min).sum()
    }
}
// end_submission
