#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
#[allow(dead_code)]
impl Solution {
    pub fn min_damage(power: i32, damage: Vec<i32>, health: Vec<i32>) -> i64 {
        let mut v: Vec<_> = health.into_iter().zip(damage).collect();
        v.sort_by(|(h1, d1), (h2, d2)| {
            ((h1 + power - 1) / power * d2).cmp(&((h2 + power - 1) / power * d1))
        });

        let mut t = 0;
        let mut c = 0i64;

        for (h, d) in v {
            t += (h + power - 1) / power;
            c += t as i64 * d as i64;
        }

        c
    }
}
// end_submission
