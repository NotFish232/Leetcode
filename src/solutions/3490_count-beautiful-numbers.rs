#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
use std::collections::HashMap;

#[allow(dead_code)]
impl Solution {
    fn digit_dp(
        pos: usize,
        cur_prod: i32,
        cur_sum: i32,
        l_num: &Vec<i32>,
        r_num: &Vec<i32>,
        l_tight: i32,
        r_tight: i32,
        dp: &mut HashMap<(usize, i32, i32, i32, i32), i32>,
    ) -> i32 {
        if pos == l_num.len() {
            return (cur_prod % cur_sum == 0) as i32;
        }

        let k = (pos, cur_prod, cur_sum, l_tight, r_tight);

        if let Some(&x) = dp.get(&k) {
            return x;
        }

        let bl = if l_tight == 1 { l_num[pos] } else { 0 };
        let ul = if r_tight == 1 { r_num[pos] } else { 9 };

        let mut res = 0;

        for d in bl..=ul {
            res += Self::digit_dp(
                pos + 1,
                cur_prod * if cur_sum == 0 && d == 0 { 1 } else { d },
                cur_sum + d,
                l_num,
                r_num,
                l_tight & (d == l_num[pos]) as i32,
                r_tight & (d == r_num[pos]) as i32,
                dp,
            );
        }

        *dp.entry(k).or_insert(res)
    }

    pub fn beautiful_numbers(l: i32, r: i32) -> i32 {
        let mut s_l = l.to_string();
        let s_r = r.to_string();

        s_l = format!("{:0>w$}", s_l, w = s_r.len());

        let v_l = s_l
            .chars()
            .map(|x| x.to_digit(10).unwrap() as i32)
            .collect();
        let v_r = s_r
            .chars()
            .map(|x| x.to_digit(10).unwrap() as i32)
            .collect();

        let mut dp = HashMap::new();

        Self::digit_dp(0, 1, 0, &v_l, &v_r, 1, 1, &mut dp)
    }
}
// end_submission
