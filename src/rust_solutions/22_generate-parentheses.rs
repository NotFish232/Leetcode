#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::collections::HashSet;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        if n == 0 {
            return vec!["".to_string()];
        }
        if n == 1 {
            return vec!["()".to_string()];
        }

        let mut hs = HashSet::new();

        for i in 0..n {
            let j = n - i - 1;

            for p1 in Solution::generate_parenthesis(i) {
                for p2 in Solution::generate_parenthesis(j) {
                    hs.extend(vec![format!("({}){}", p1, p2), format!("{}({})", p1, p2)]);
                }
            }
        }

        hs.into_iter().collect()
    }
}
// end_submission
