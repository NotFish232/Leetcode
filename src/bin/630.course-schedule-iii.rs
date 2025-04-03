#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
use std::collections::BinaryHeap;

#[allow(dead_code)]
impl Solution {
    pub fn schedule_course(mut courses: Vec<Vec<i32>>) -> i32 {
        courses.sort_by_key(|x| x[1]);

        let mut v = BinaryHeap::new();
        let mut t = 0;
        for c in courses {
            v.push(c[0]);
            t += c[0];

            if t > c[1] {
                t -= v.pop().unwrap();
            }
        }

        v.len() as i32
    }
}
// end_submission
