#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::cmp::{max, min};
impl Solution {
    pub fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let mut min_val = 0;
        let mut max_val = 0;

        let mut c = 0;
        for d in differences {
            c += d;

            min_val = min(min_val, c);
            max_val = max(max_val, c);

            if max_val - min_val >= upper - lower + 1 {
                return 0;
            }
        }

        (upper - lower) - (max_val - min_val) + 1
    }
}
// end_submission
