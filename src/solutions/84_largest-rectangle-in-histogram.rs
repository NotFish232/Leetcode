#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::cmp::max;

impl Solution {
    pub fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
        heights.push(0);

        let mut s = Vec::new();
        let mut max_height = -1;

        for (i, h) in heights.into_iter().enumerate() {
            let mut initial_i = i;
            while let Some(&(o_h, j)) = s.last() {
                if o_h >= h {
                    max_height = max(max_height, (i - j) as i32 * o_h);
                    initial_i = j;
                    s.pop();
                } else {
                    break;
                }
            }
            
            s.push((h, initial_i));
        }

        max_height
    }
}
// end_submission
