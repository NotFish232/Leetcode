use std::cmp::max;

impl Solution {
    pub fn rob(mut nums: Vec<i32>) -> i32 {
        let mut p = vec![0; nums.len() + 1];
        p[1] = nums[0];
        for i in (1..nums.len()) {
            p[i + 1] = max(p[i - 1] + nums[i], p[i]);
        }
        return p[nums.len()];
    }
}
