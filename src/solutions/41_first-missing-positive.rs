#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let mut i = 0;
        while i < nums.len() {
            if nums[i] <= 0 || nums[i] > nums.len() as i32 {
                nums[i] = -1;
                i += 1;
            } else if i + 1 == nums[i] as usize {
                i += 1;
            } else {
                let n = (nums[i] - 1) as usize;
                nums.swap(i, n);

                if nums[i] == nums[n] {
                    nums[i] = -1;
                }

                if nums[i] == -1 {
                    i += 1;
                }
            }
        }

        let mut i = 1;
        while i - 1 < nums.len() && nums[i - 1] != -1 {
            i += 1;
        }

        i as i32
    }
}
// end_submission
