#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .filter(|&x| (x as f32).log10().floor() as i32 % 2 == 1)
            .count() as i32
    }
}
// end_submission
