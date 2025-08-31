#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    fn count(nums: &[i32], len: i32, val: i32) -> i32 {
        let mut res = 0;

        let mut cur_count = 0;

        for &n in nums.iter().chain(&[i32::MAX]) {
            if n <= val {
                cur_count += 1;
            } else {
                res += cur_count / len;
                cur_count = 0;
            }
        }

        res
    }

    pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        let (mut l, mut r) = (1, *bloom_day.iter().max().unwrap());

        let mut ans = -1;

        while l <= r {
            let mid = l + (r - l) / 2;

            if Self::count(&bloom_day, k, mid) >= m {
                ans = mid;
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }

        ans
    }
}
// end_submission
