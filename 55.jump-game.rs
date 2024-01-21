use std::cmp::min;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut v = vec![false; nums.len()];
        v[0] = true;

        for (i, &n) in nums.iter().enumerate() {
            if v[i] {
                for j in (i + 1..min(i + n as usize + 1, nums.len())) {
                    v[j] = true;
                }
            }
        }

        v[v.len() - 1]
    }
}
