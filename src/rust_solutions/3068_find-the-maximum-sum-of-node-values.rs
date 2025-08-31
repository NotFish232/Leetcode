#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission

impl Solution {
    #[allow(unused)]
    pub fn maximum_value_sum(nums: Vec<i32>, k: i32, edges: Vec<Vec<i32>>) -> i64 {
        let mut diffs: Vec<_> = nums
            .iter()
            .enumerate()
            .map(|(i, &n)| ((n ^ k) - n, i))
            .collect();
        diffs.sort();

        let mut i = nums.len() - 1;
        let mut s = 0;

        while i != usize::MAX {
            if i > 0
                && ((diffs[i].0 > 0 && diffs[i - 1].0 > 0)
                    || (diffs[i].0 > 0 && diffs[i].0 > -diffs[i - 1].0))
            {
                s += (nums[diffs[i].1] ^ k) as i64;
                s += (nums[diffs[i - 1].1] ^ k) as i64;
                i -= 2;
            } else {
                s += nums[diffs[i].1] as i64;
                i -= 1;
            }
        }

        s
    }
}
// end_submission
