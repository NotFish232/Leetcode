#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::{cmp::max, collections::VecDeque};

impl Solution {
    pub fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
        let n = colors.len();

        let mut adj = vec![Vec::new(); n];
        let mut in_degrees = vec![0; n];

        for e in edges {
            adj[e[0] as usize].push(e[1] as usize);
            in_degrees[e[1] as usize] += 1;
        }

        let mut q: VecDeque<usize> = VecDeque::from(
            in_degrees
                .iter()
                .enumerate()
                .filter_map(|(i, &x)| if x == 0 { Some(i) } else { None })
                .collect::<Vec<_>>(),
        );

        let mut dp = vec![vec![0; 26]; n];

        let mut seen = 0;
        let mut ans = 0;

        while let Some(n) = q.pop_front() {
            seen += 1;

            let c = (colors.as_bytes()[n] - b'a') as usize;

            dp[n][c] += 1;

            ans = max(ans, dp[n][c]);

            for &other_n in &adj[n] {
                for i in 0..26 {
                    dp[other_n][i] = max(dp[other_n][i], dp[n][i]);
                }
                in_degrees[other_n] -= 1;
                if in_degrees[other_n] == 0 {
                    q.push_back(other_n);
                }
            }
        }

        if seen == n { ans } else { -1 }
    }
}
// end_submission
