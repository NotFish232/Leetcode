#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::collections::{HashMap, hash_map::Entry};

impl Solution {
    fn digit_dp(
        pos: usize,
        num_ones: i32,
        tight: bool,
        digits: &[i32],
        dp: &mut HashMap<(usize, i32, bool), i32>,
    ) -> i32 {
        if pos == digits.len() {
            return num_ones;
        }

        let dp_key = (pos, num_ones, tight);

        if let Entry::Occupied(e) = dp.entry(dp_key) {
            return *e.get();
        }

        let mut c = 0;

        let ub = if tight { digits[pos] } else { 9 };

        for d in 0..=ub {
            c += Self::digit_dp(
                pos + 1,
                num_ones + (d == 1) as i32,
                tight && d == digits[pos],
                digits,
                dp,
            );
        }

        dp.insert(dp_key, c);

        c
    }

    pub fn count_digit_one(n: i32) -> i32 {
        let mut dp = HashMap::new();

        Self::digit_dp(
            0,
            0,
            true,
            &n.to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<_>>(),
            &mut dp,
        )
    }
}
// end_submission
