#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::collections::HashMap;

impl Solution {
    fn digit_dp(
        idx: usize,
        l_tight: bool,
        r_tight: bool,
        delta: i32,
        remainder: i32,
        has_nonzero: bool,
        l_digits: &[i32],
        r_digits: &[i32],
        k: i32,
        dp: &mut HashMap<(usize, bool, bool, i32, i32), i32>,
    ) -> i32 {
        if idx == l_digits.len() {
            return (delta == 0 && remainder % k == 0) as i32;
        }

        let dp_key = (idx, l_tight, r_tight, delta, remainder);

        if let Some(res) = dp.get(&dp_key) {
            return *res;
        }

        let mut total = 0;

        let l_lim = if l_tight { l_digits[idx] } else { 0 };
        let r_lim = if r_tight { r_digits[idx] } else { 9 };

        for d in l_lim..=r_lim {
            total += Self::digit_dp(
                idx + 1,
                l_tight && (d == l_digits[idx]),
                r_tight && (d == r_digits[idx]),
                if has_nonzero || d != 0 {
                    delta + if d % 2 == 0 { 1 } else { -1 }
                } else {
                    0
                },
                (remainder * 10 + d) % k,
                has_nonzero || d != 0,
                l_digits,
                r_digits,
                k,
                dp,
            )
        }

        dp.insert(dp_key, total);

        total
    }

    pub fn number_of_beautiful_integers(low: i32, high: i32, k: i32) -> i32 {
        let mut s_l = low.to_string();
        let s_r = high.to_string();

        s_l = format!("{:0>w$}", s_l, w = s_r.len());

        let v_l: Vec<_> = s_l
            .chars()
            .map(|x| x.to_digit(10).unwrap() as i32)
            .collect();
        let v_r: Vec<_> = s_r
            .chars()
            .map(|x| x.to_digit(10).unwrap() as i32)
            .collect();

        let mut dp = HashMap::new();

        Self::digit_dp(0, true, true, 0, 0, false, &v_l, &v_r, k, &mut dp)
    }
}
// end_submission
