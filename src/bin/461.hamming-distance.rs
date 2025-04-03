#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
use std::cmp::max;

#[allow(dead_code)]
impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let n = (max(x, y) as f32 + 1f32).log2().ceil() as i32;

        (0..n)
            .map(|i| (((x >> i) & 1) != ((y >> i) & 1)) as i32)
            .sum()
    }
}
// end_submission
