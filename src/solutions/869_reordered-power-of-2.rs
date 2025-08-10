#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::collections::HashSet;

impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        let mut h: HashSet<String> = HashSet::new();

        let mut i = 0;
        while 2i32.pow(i) < 10i32.pow(9) {
            let mut v: Vec<_> = 2i32.pow(i).to_string().chars().collect();
            v.sort();
 
            h.insert(v.iter().collect());

            i += 1;
        }

        let mut v: Vec<_> = n.to_string().chars().collect();
        v.sort();

        h.contains(&v.iter().collect::<String>())
    }
}
// end_submission
