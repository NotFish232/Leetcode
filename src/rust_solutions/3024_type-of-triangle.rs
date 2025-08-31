#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::collections::HashSet;

impl Solution {
    pub fn triangle_type(nums: Vec<i32>) -> String {
        for i in 0..nums.len() {
            let (j, k) = ((i + 1) % 3, (i + 2) % 3);

            if nums[i] >= nums[j] + nums[k] {
                return "none".to_string();
            }
        }

        let h: HashSet<_> = nums.iter().collect();

        match h.len() {
            1 => "equilateral",
            2 => "isosceles",
            _ => "scalene",
        }
        .to_string()
    }
}
// end_submission
