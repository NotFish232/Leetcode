#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
#[allow(dead_code)]
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1, -1];
        }

        let mut left_ptr = 0;
        let mut right_ptr = nums.len() - 1;

        while left_ptr <= right_ptr {
            let mid_ptr = (left_ptr + right_ptr) / 2;

            if nums[mid_ptr] == target {
                left_ptr = mid_ptr;
                right_ptr = mid_ptr;

                while left_ptr > 0 && nums[left_ptr - 1] == target {
                    left_ptr -= 1;
                }
                while right_ptr + 1 < nums.len() && nums[right_ptr + 1] == target {
                    right_ptr += 1;
                }

                return vec![left_ptr as i32, right_ptr as i32];
            }

            if nums[mid_ptr] > target {
                if mid_ptr == 0 {
                    break;
                }
                right_ptr = mid_ptr - 1;
            } else {
                left_ptr = mid_ptr + 1;
            }
        }

        vec![-1, -1]
    }
}
// end_submission
