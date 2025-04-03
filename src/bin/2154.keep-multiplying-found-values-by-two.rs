#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
#[allow(dead_code)]
impl Solution {
    pub fn find_final_value(nums: Vec<i32>, mut original: i32) -> i32 {
        while nums.contains(&original) {
            original *= 2;
        }

        original
    }
}
// end_submission
