#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::collections::HashMap;

impl Solution {
    pub fn max_subarrays(n: i32, conflicting_pairs: Vec<Vec<i32>>) -> i64 {
        let n = n as usize;

        let mut left_bounds = HashMap::new();

        for p in conflicting_pairs {
            left_bounds
                .entry(p[0].max(p[1]) as usize)
                .or_insert(Vec::new())
                .push(p[0].min(p[1]) as usize);
        }

        let mut l_mins = vec![0usize; 2];
        let mut c = 0i64;
        let mut g = HashMap::new();

        for i in 1..=n {
            if let Some(lb) = left_bounds.get(&i) {
                for &l in lb {
                    l_mins.push(l);
                    l_mins.sort();
                    l_mins.remove(0);
                }
            }

            c += (i - l_mins[1]) as i64;
            *g.entry(l_mins[1]).or_insert(0) += (l_mins[1] - l_mins[0]) as i64
        }

        c + g.values().max().unwrap()
    }
}
// end_submission
