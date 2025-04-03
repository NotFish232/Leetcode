#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
use std::collections::VecDeque;

#[allow(dead_code)]
impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let mut q = VecDeque::from(bits);
        while !q.is_empty() {
            if q[0] == 0 {
                if q.len() == 1 {
                    return true;
                }
                q.pop_front();
            } else if q.len() > 1 {
                q.pop_front();
                q.pop_front();
            } else {
                break;
            }
        }

        false
    }
}
// end_submission
