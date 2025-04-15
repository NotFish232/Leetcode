#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        let mut v = Vec::new();

        for (n, i) in nums.into_iter().zip(index.into_iter()) {
            v.insert(i as usize, n);
        }

        v
    }
}
// end_submission
