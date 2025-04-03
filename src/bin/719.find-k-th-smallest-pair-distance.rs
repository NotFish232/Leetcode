#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
#[allow(dead_code)]
impl Solution {
    fn num_pairs_within_dist(nums: &[i32], dist: i32) -> i32 {
        let (mut l, mut r) = (0, 1);
        let mut count = 0;

        while r < nums.len() {
            while nums[r] - nums[l] > dist {
                l += 1;
            }

            count += (r - l) as i32;

            r += 1;
        }

        count
    }

    pub fn smallest_distance_pair(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();

        let (mut l, mut r) = (0, nums[nums.len() - 1] - nums[0]);

        while l < r {
            let m = l + (r - l) / 2;

            if Self::num_pairs_within_dist(&nums, m) >= k {
                r = m;
            } else {
                l = m + 1;
            }
        }

        l
    }
}
// end_submission
