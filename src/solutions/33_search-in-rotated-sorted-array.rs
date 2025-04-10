#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
#[allow(dead_code)]
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left_ptr = 0;
        let mut right_ptr = nums.len() - 1;

        while left_ptr <= right_ptr {
            let mid_ptr = (left_ptr + right_ptr) / 2;

            if nums[mid_ptr] == target {
                return mid_ptr as i32;
            }

            if nums[left_ptr] <= nums[mid_ptr] {
                if nums[left_ptr] <= target && target < nums[mid_ptr] {
                    right_ptr = mid_ptr - 1;
                } else {
                    left_ptr = mid_ptr + 1;
                }
            } else if nums[mid_ptr] < target && target <= nums[right_ptr] {
                left_ptr = mid_ptr + 1;
            } else {
                right_ptr = mid_ptr - 1;
            }
        }

        -1
    }
}
// end_submission
