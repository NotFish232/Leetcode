#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut idx = 0;

        while idx < nums.len() {
            if idx > 0 && nums[idx] == nums[idx - 1] {
                nums.remove(idx);
                idx -= 1;
            }
            idx += 1;
        }

        idx as i32
    }
}
// end_submission
