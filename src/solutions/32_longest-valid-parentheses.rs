#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::cmp::max;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut v = vec![0; s.len() + 1];

        let mut stack = Vec::new();

        for (i, c) in s.chars().enumerate() {
            if c == '(' {
                stack.push((c, i))
            } else if let Some(x) = stack.pop() {
                if x.0 == '(' {
                    v[i] = 1;
                    v[x.1] = 1;
                }
            }
        }

        let mut cur_len = 0;
        let mut max_len = 0;

        for x in v {
            if x == 1 {
                cur_len += 1;
            } else {
                max_len = max(max_len, cur_len);
                cur_len = 0;
            }
        }

        max_len
    }
}
// end_submission
