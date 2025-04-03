#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
use std::cmp::min;

#[allow(dead_code)]
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut v = vec![i32::MAX; nums.len()];
        v[0] = 0;

        for (i, &n) in nums.iter().enumerate() {
            for j in i + 1..min(i + n as usize + 1, nums.len()) {
                v[j] = min(v[j], v[i] + 1);
            }
        }

        v[v.len() - 1]
    }
}
// end_submission
