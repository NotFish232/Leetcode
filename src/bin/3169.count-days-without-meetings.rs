#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(dead_code)]
impl Solution {
    pub fn count_days(days: i32, meetings: Vec<Vec<i32>>) -> i32 {
        let mut h = BinaryHeap::new();

        for m in meetings {
            h.push((Reverse(m[0]), 1));
            h.push((Reverse(m[1]), -1))
        }
        h.push((Reverse(days + 1), 1));

        let mut count = 0;

        let mut d = 0;
        let mut c = 0;

        while let Some((Reverse(t), b)) = h.pop() {
            if c == 0 {
                count += t - d - 1;
            }

            c += b;
            d = t;
        }

        count
    }
}
// end_submission
