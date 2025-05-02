#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    fn solve(cur: &mut Vec<i32>, nums: &[i32], idx: usize, v: &mut Vec<Vec<i32>>) {
        if idx == nums.len() {
            v.push(cur.clone());
            return;
        }

        Self::solve(cur, nums, idx + 1, v);
        cur.push(nums[idx]);
        Self::solve(cur, nums, idx + 1, v);
        cur.pop();
    }

    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut v = Vec::new();

        Self::solve(&mut Vec::new(), &nums, 0, &mut v);

        v
    }
}
// end_submission
