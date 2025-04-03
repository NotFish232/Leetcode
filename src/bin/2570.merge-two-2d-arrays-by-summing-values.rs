#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
#[allow(dead_code)]
impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut v = Vec::new();

        let (mut i1, mut i2) = (0, 0);

        while i1 < nums1.len() || i2 < nums2.len() {
            if i1 == nums1.len() {
                v.push(nums2[i2].clone());
                i2 += 1;
            } else if i2 == nums2.len() {
                v.push(nums1[i1].clone());
                i1 += 1;
            } else if nums1[i1][0] < nums2[i2][0] {
                v.push(nums1[i1].clone());
                i1 += 1;
            } else if nums2[i2][0] < nums1[i1][0] {
                v.push(nums2[i2].clone());
                i2 += 1;
            } else {
                v.push(vec![nums1[i1][0], nums1[i1][1] + nums2[i2][1]]);
                i1 += 1;
                i2 += 1;
            }
        }

        v
    }
}
// end_submission
