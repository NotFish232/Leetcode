#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::{cmp::min, collections::HashMap};

impl Solution {
    fn create_lps(s: &[i32]) -> Vec<usize> {
        let mut lps = vec![0; s.len()];

        let mut i = 1;
        let mut len = 0;

        while i < s.len() {
            if s[i] == s[len] {
                len += 1;
                lps[i] = len;
                i += 1;
            } else if len != 0 {
                len = lps[len - 1];
            } else {
                i += 1;
            }
        }

        lps
    }

    fn next_match(s: &[i32], s_idx: usize, d: i32, lps: &[usize]) -> usize {
        let mut idx = s_idx;

        if idx == s.len() {
            idx = lps[idx - 1];
        }

        while idx > 0 && s[idx] != d {
            idx = lps[idx - 1];
        }

        if s[idx] == d { idx + 1 } else { 0 }
    }

    fn digit_dp(
        n_idx: usize,
        s_idx: usize,
        l_tight: i32,
        r_tight: i32,
        limit: i32,
        left: &[i32],
        right: &[i32],
        s: &[i32],
        lps: &[usize],
        dp: &mut HashMap<(usize, usize, i32, i32), i64>,
    ) -> i64 {
        if n_idx == left.len() {
            return (s_idx == s.len()) as i64;
        }

        let lb = if l_tight == 1 { left[n_idx] } else { 0 };
        let ub = if r_tight == 1 {
            min(right[n_idx], limit)
        } else {
            limit
        };

        let dp_key = (n_idx, s_idx, l_tight, r_tight);

        if let Some(val) = dp.get(&dp_key) {
            return *val;
        }

        let mut res = 0;

        for d in lb..=ub {
            res += Self::digit_dp(
                n_idx + 1,
                Self::next_match(s, s_idx, d, lps),
                l_tight & (d == left[n_idx]) as i32,
                r_tight & (d == right[n_idx]) as i32,
                limit,
                left,
                right,
                s,
                lps,
                dp,
            )
        }

        dp.insert(dp_key, res);

        res
    }

    pub fn number_of_powerful_int(start: i64, finish: i64, limit: i32, s: String) -> i64 {
        let mut dp = HashMap::new();

        let mut s_l = start.to_string();
        let s_r = finish.to_string();

        s_l = format!("{:0>w$}", s_l, w = s_r.len());

        let v_l: Vec<i32> = s_l
            .chars()
            .map(|x| x.to_digit(10).unwrap() as i32)
            .collect();
        let v_r: Vec<i32> = s_r
            .chars()
            .map(|x| x.to_digit(10).unwrap() as i32)
            .collect();

        let v_s: Vec<i32> = s.chars().map(|x| x.to_digit(10).unwrap() as i32).collect();

        let lps = Self::create_lps(&v_s);

        Self::digit_dp(0, 0, 1, 1, limit, &v_l, &v_r, &v_s, &lps, &mut dp)
    }
}
// end_submission
