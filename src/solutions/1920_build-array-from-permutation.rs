#[allow(unused)]
use crate::stubs::*;

struct Solution;

 // start_submission
impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let mut v = vec![0; nums.len()];

        for (i, &n) in nums.iter().enumerate() {
            v[i] = nums[n as usize];
        }

        v
    }
}
 // end_submission
