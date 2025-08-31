#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
        let m = str1.len();
        let n = str2.len();

        let chars1: Vec<_> = str1.chars().collect();
        let chars2: Vec<_> = str2.chars().collect();

        let mut dp = vec![vec![0; n + 1]; m + 1];

        for (i, ch1) in chars1.iter().enumerate() {
            for (j, ch2) in chars2.iter().enumerate() {
                if ch1 == ch2 {
                    dp[i + 1][j + 1] = dp[i][j] + 1;
                } else {
                    dp[i + 1][j + 1] = dp[i][j + 1].max(dp[i + 1][j]);
                }
            }
        }

        let mut v = Vec::new();
        let mut i = m;
        let mut j = n;

        while i != 0 || j != 0 {
            if i == 0 {
                v.push(chars2[j - 1]);
                j -= 1;
            } else if j == 0 {
                v.push(chars1[i - 1]);
                i -= 1;
            } else if chars1[i - 1] == chars2[j - 1] {
                v.push(chars1[i - 1]);
                i -= 1;
                j -= 1;
            } else if dp[i - 1][j] > dp[i][j - 1] {
                v.push(chars1[i - 1]);
                i -= 1;
            } else {
                v.push(chars2[j - 1]);
                j -= 1;
            }
        }

        v.into_iter().rev().collect()
    }
}
// end_submission
