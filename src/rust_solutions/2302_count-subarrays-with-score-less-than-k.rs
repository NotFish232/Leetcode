#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        let n = nums.len();

        let mut count = 0;
        let mut cur_sum = 0;

        let (mut l, mut r) = (0, 0);

        while r < nums.len() {
            cur_sum += nums[r] as i64;
            while cur_sum * (r - l + 1) as i64 >= k {
                count += (nums.len() - r) as i64;

                cur_sum -= nums[l] as i64;
                l += 1;
            }

            r += 1;
        }

        (n * (n + 1) / 2) as i64 - count
    }
}
// end_submission
