#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
#[allow(dead_code)]
impl Solution {
    pub fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
        let (mut l, mut r) = (1, *nums.iter().max().unwrap());

        while l < r {
            let mid = l + (r - l) / 2;

            let mut count = 0;

            let mut i = 0;
            while i < nums.len() {
                if nums[i] <= mid {
                    count += 1;
                    i += 1;
                }
                i += 1;
            }

            if count >= k {
                r = mid;
            } else {
                l = mid + 1;
            }
        }

        l
    }
}
// end_submission
