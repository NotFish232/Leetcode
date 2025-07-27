#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::cmp::Reverse;

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut cars: Vec<_> = position.into_iter().zip(speed).collect();
        cars.sort_by_key(|x| Reverse(x.0));

        let mut stk = Vec::new();

        for (p, s) in cars {
            if let Some(&(o_p, o_s)) = stk.last() {
                if (target - p) as f32 / s as f32 > (target - o_p) as f32 / o_s as f32 {
                    stk.push((p, s))
                }
            } else {
                stk.push((p, s));
            }
        }

        stk.len() as i32
    }
}
// end_submission
