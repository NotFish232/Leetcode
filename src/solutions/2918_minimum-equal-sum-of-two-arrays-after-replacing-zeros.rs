#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::cmp::max;

impl Solution {
    pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let n1_sum: i64 = nums1.iter().map(|&n| n as i64).sum();
        let n1_count = nums1.iter().filter(|&&n| n == 0).count() as i64;

        let n2_sum: i64 = nums2.iter().map(|&n| n as i64).sum();
        let n2_count = nums2.iter().filter(|&&n| n == 0).count() as i64;

        let res = max(n1_sum + n1_count, n2_sum + n2_count);

        if (n1_sum == n2_sum && n1_count + n2_count == 0)
            || (n2_count > 0 && n2_count <= n1_sum - n2_sum + n1_count)
            || (n1_count > 0 && n1_count <= n2_sum - n1_sum + n2_count)
        {
            res
        } else {
            -1
        }
    }
}
// end_submission
