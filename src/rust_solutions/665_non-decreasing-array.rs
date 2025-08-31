#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn check_possibility(mut nums: Vec<i32>) -> bool {
        let mut p = false;

        for i in 0..nums.len() - 1 {
            if nums[i + 1] < nums[i] {
                if p {
                    return false;
                }

                if i == 0 || nums[i - 1] <= nums[i + 1] {
                    nums[i] = nums[i + 1];
                } else {
                    nums[i + 1] = nums[i];
                }
                p = true;
            }
        }

        true
    }
}
// end_submission
