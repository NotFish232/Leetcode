#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::cmp::max;

impl Solution {
    pub fn get_words_in_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let n = words.len();

        let mut dists = vec![vec![-1; n]; n];
        for (i, w_1) in words.iter().enumerate() {
            for (j, w_2) in words.iter().enumerate() {
                if w_1.len() == w_2.len() {
                    dists[i][j] = w_1
                        .chars()
                        .zip(w_2.chars())
                        .filter(|(c_1, c_2)| c_1 != c_2)
                        .count() as i32;
                }
            }
        }

        let mut dp = vec![vec![0; n]; n];
        for i in 0..n {
            dp[i][i] = 1;
        }

        for i in 0..n {
            for j in 0..i {
                if dists[i][j] == 1 && groups[i] != groups[j] {
                    dp[i][j] = max(dp[i][j], dp[j].iter().max().unwrap() + 1);
                }
            }
        }

        let mut idx = dp
            .iter()
            .enumerate()
            .max_by_key(|(_, v)| v.iter().max().unwrap())
            .unwrap()
            .0;

        let mut res = Vec::new();

        loop {
            res.push(words[idx].clone());

            let max_idx = dp[idx]
                .iter()
                .enumerate()
                .max_by_key(|(_, x)| **x)
                .unwrap()
                .0;

            if dp[idx][max_idx] == 1 {
                break;
            }

            idx = max_idx;
        }

        res.reverse();

        res
    }
}
// end_submission
