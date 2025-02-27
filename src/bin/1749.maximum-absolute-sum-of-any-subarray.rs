use std::cmp::{max, min};

impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let mut cur_max_pos = nums[0];
        let mut cur_max_neg = nums[0];
        let mut max_abs = nums[0].abs();

        for num in nums.into_iter().skip(1) {
            cur_max_pos = max(cur_max_pos + num, num);
            cur_max_neg = min(cur_max_neg + num, num);

            max_abs = max(max_abs, max(cur_max_pos.abs(), cur_max_neg.abs()));
        }
        max_abs
    }
}
