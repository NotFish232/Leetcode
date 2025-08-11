#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::collections::{hash_map::Entry, HashMap};

impl Solution {
    fn result(c: i32, n: i32, mask: i32, dp: &mut HashMap<i32, bool>) -> bool {
        if c <= 0 {
            return false;
        }

        if let Entry::Occupied(b) = dp.entry(mask) {
            return *b.get();
        }

        let mut side_res = false;

        for i in 1..=n {
            if mask & (1 << i) == 0 {
                let res = Solution::result(c - i, n, mask + (1 << i), dp);

                if !res {
                    side_res = true;
                }
            }
        }

        dp.insert(mask, side_res);

        side_res
    }

    pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
        if desired_total == 0 {
            true
        } else if ((max_choosable_integer + 1) * max_choosable_integer) / 2 < desired_total {
            false
        } else {
            let mut dp = HashMap::new();

            Solution::result(desired_total, max_choosable_integer, 0, &mut dp)
        }
    }
}
// end_submission
