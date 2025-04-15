#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        nums.sort();

        let mut quadruplets = Vec::new();

        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            for j in i + 1..nums.len() {
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }

                let mut left_ptr = j + 1;
                let mut right_ptr = nums.len() - 1;

                while left_ptr < right_ptr {
                    let sum = nums[i] as i64
                        + nums[j] as i64
                        + nums[left_ptr] as i64
                        + nums[right_ptr] as i64;
                    let target_64 = target as i64;

                    if sum > target_64 {
                        right_ptr -= 1;
                    } else if sum < target_64 {
                        left_ptr += 1;
                    } else {
                        quadruplets.push(vec![nums[i], nums[j], nums[left_ptr], nums[right_ptr]]);

                        left_ptr += 1;

                        while left_ptr < right_ptr && nums[left_ptr] == nums[left_ptr - 1] {
                            left_ptr += 1;
                        }
                    }
                }
            }
        }

        quadruplets
    }
}
// end_submission
