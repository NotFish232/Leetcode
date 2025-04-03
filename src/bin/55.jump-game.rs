#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
use std::cmp::min;

#[allow(dead_code)]
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut v = vec![false; nums.len()];
        v[0] = true;

        for (i, &n) in nums.iter().enumerate() {
            if v[i] {
                for j in i + 1..min(i + n as usize + 1, nums.len()) {
                    v[j] = true;
                }
            }
        }

        v[v.len() - 1]
    }
}
// end_submission
