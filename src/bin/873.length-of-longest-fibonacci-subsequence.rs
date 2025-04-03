#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
use std::collections::HashMap;

#[allow(dead_code)]
impl Solution {
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        let mut dp = vec![vec![2; arr.len()]; arr.len()];
        let mut val_idxs: HashMap<_, usize> = HashMap::new();
        let mut longest_subseq = i32::MIN;

        for i in 0..arr.len() {
            for j in 0..i {
                let prev_seq_val = arr[i] - arr[j];

                if prev_seq_val < arr[j] && val_idxs.contains_key(&prev_seq_val) {
                    dp[j][i] = dp[j][i].max(dp[*val_idxs.get(&prev_seq_val).unwrap()][j] + 1);

                    longest_subseq = longest_subseq.max(dp[j][i]);
                }
            }

            val_idxs.insert(arr[i], i);
        }

        longest_subseq.max(0)
    }
}
// end_submission
