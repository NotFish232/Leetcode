#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
#[allow(dead_code)]
impl Solution {
    pub fn next_permutation(nums: &mut [i32]) {
        let n = nums.len();

        for i in (0..n - 1).rev() {
            if nums[i] < nums[i + 1] {
                let s = &nums[i + 1..n];
                let min_val = s.iter().filter(|&&x| x > nums[i]).min().unwrap();
                let min_idx = i + 1 + s.iter().position(|x| x == min_val).unwrap();

                nums.swap(i, min_idx);
                nums[i + 1..n].sort();

                return;
            }
        }

        nums.sort();
    }
}
// end_submission
