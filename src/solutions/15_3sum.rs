#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();

        let mut triplets = Vec::new();

        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut left_ptr = i + 1;
            let mut right_ptr = nums.len() - 1;

            while left_ptr < right_ptr {
                let sum = nums[i] + nums[left_ptr] + nums[right_ptr];

                if sum > 0 {
                    right_ptr -= 1;
                } else if sum < 0 {
                    left_ptr += 1;
                } else {
                    triplets.push(vec![nums[i], nums[left_ptr], nums[right_ptr]]);

                    while left_ptr < right_ptr && nums[i] + nums[left_ptr] + nums[right_ptr] == 0 {
                        left_ptr += 1;
                    }
                }
            }
        }

        triplets
    }
}
// end_submission
