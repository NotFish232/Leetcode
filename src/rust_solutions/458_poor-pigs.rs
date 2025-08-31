#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        let num_tests = minutes_to_test / minutes_to_die;
        let mut c = 0;

        while (num_tests + 1).pow(c) < buckets {
            c += 1;
        }

        c as i32
    }
}
// end_submission
