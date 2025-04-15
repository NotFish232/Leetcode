#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::{
    cmp::{Ordering, max},
    iter::repeat_n,
};

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let mut v1_parts: Vec<i32> = version1.split('.').map(|x| x.parse().unwrap()).collect();
        let mut v2_parts: Vec<i32> = version2.split('.').map(|x| x.parse().unwrap()).collect();

        let n = max(v1_parts.len(), v2_parts.len());

        v1_parts.extend(repeat_n(0, n - v1_parts.len()));
        v2_parts.extend(repeat_n(0, n - v2_parts.len()));

        match v1_parts.cmp(&v2_parts) {
            Ordering::Less => -1,
            Ordering::Greater => 1,
            Ordering::Equal => 0,
        }
    }
}
// end_submission
