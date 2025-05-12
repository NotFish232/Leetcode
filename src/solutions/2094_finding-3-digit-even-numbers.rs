#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::collections::HashSet;

impl Solution {
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let mut h = HashSet::new();

        for i in 0..digits.len() {
            for j in 0..digits.len() {
                for k in 0..digits.len() {
                    if i == j || i == k || j == k {
                        continue;
                    }
                    
                    if digits[i] != 0 && digits[k] % 2 == 0 {
                        let d = digits[i] * 100 + digits[j] * 10 + digits[k];
                        h.insert(d);
                    }
                }
            }
        }

        let mut v: Vec<_> = h.into_iter().collect();
        v.sort();

       v
    }
}
// end_submission
