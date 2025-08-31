#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::collections::HashSet;

impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        let mut m = 0;
        let mut n = 0;

        let mut seen_m = HashSet::new();

        while !seen_m.contains(&m) {
            seen_m.insert(m);
            m = (10 * m + 1) % k;
            n += 1;

            if m == 0 {
                return n;
            }
        }

        -1
    }
}
// end_submission
