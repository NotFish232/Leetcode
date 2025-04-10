#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
#[allow(dead_code)]
impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut cur_count = 0;

        for i in 2..nums.len() {
            if nums[i] - nums[i - 1] == nums[i - 1] - nums[i - 2] {
                cur_count += 1;
                count += cur_count;
            } else {
                cur_count = 0;
            }
        }

        count
    }
}
// end_submission
