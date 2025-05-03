#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::cmp::min;

impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        for i in 1..=6 {
            'out: {
                let (mut t_count, mut b_count) = (0, 0);
                
                for (&t, &b) in tops.iter().zip(&bottoms) {
                    if t != i && b != i {
                        break 'out;
                    }

                    if t != i {
                        b_count += 1;
                    }
                    if b != i {
                        t_count += 1;
                    }
                }

                return min(t_count, b_count);
            }
        }

        -1
    }
}
// end_submission
