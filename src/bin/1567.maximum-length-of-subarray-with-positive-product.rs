#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
use std::cmp::min;

#[allow(dead_code)]
impl Solution {
    pub fn get_max_len(nums: Vec<i32>) -> i32 {
        nums.split(|&x| x == 0)
            .map(|v| {
                let parity = v.iter().filter(|&&x| x < 0).count();

                if parity % 2 == 0 {
                    v.len()
                } else {
                    v.len()
                        - min(
                            v.iter().position(|&x| x < 0).unwrap(),
                            v.iter().rev().position(|&x| x < 0).unwrap(),
                        )
                        - 1
                }
            })
            .max()
            .unwrap() as i32
    }
}
// end_submission
