#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn xor_game(nums: Vec<i32>) -> bool {
        nums.iter().fold(0, |acc, x| acc ^ x) == 0 || nums.len() % 2 == 0
    }
}
// end_submission
