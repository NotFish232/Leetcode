#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    fn lower_bound(nums: &[i32], val: i32) -> i64 {
        let (mut l_ptr, mut r_ptr) = (0, nums.len() - 1);
        let mut count = 0;

        while l_ptr < r_ptr {
            let sum = nums[l_ptr] + nums[r_ptr];

            if sum < val {
                count += (r_ptr - l_ptr) as i64;
                l_ptr += 1;
            } else {
                r_ptr -= 1;
            }
        }

        count
    }
    pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        nums.sort();

        Self::lower_bound(&nums, upper + 1) - Self::lower_bound(&nums, lower)
    }
}
// end_submission
