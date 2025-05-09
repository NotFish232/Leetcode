#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::cmp::{max, min};

impl Solution {
    const MOD: i64 = 1_000_000_007;

    pub fn count_balanced_permutations(num: String) -> i32 {
        let n = num.len();
        let mut counts = [0; 10];
        let mut sum = 0;

        for c in num.chars() {
            let d = c.to_digit(10).unwrap();
            counts[d as usize] += 1;
            sum += d as usize;
        }

        if sum % 2 != 0 {
            return 0;
        }

        let target = sum / 2;

        let max_odd = n.div_ceil(2);
        let mut comb = vec![vec![0; max_odd + 1]; max_odd + 1];
        

        for i in 0..=max_odd {
            comb[i][i] = 1;
            comb[i][0] = 1;
            for j in 1..i {
                comb[i][j] = (comb[i - 1][j] + comb[i - 1][j - 1]) % Self::MOD;
            }
        }

        let mut f = vec![vec![0; max_odd + 1]; target + 1];
        f[0][0] = 1;
        
        let mut p_sum = 0;
        let mut t_sum = 0;

        for i in 0..=9 {
            p_sum += counts[i];
            t_sum += i * counts[i];

            for odd_counts in (max(0, p_sum as i32 - (n as i32 - max_odd as i32)) as usize
                ..=min(p_sum, max_odd))
                .rev()
            {
                let even_counts = p_sum - odd_counts;
                for curr in (max(0, t_sum as i32 - target as i32) as usize
                    ..=min(t_sum, target))
                    .rev()
                {
                    let mut res = 0;
                    for j in (max(0, counts[i] as i32 - even_counts as i32) as usize
                        ..=min(counts[i], odd_counts))
                        .rev()
                    {
                        if i * j <= curr {
                            let ways = (comb[odd_counts][j] * comb[even_counts][counts[i] - j])
                                % Self::MOD;
                            res = (res + ways * f[curr - i * j][odd_counts - j]) % Self::MOD;
                        }
                    }
                    f[curr][odd_counts] = res % Self::MOD;
                }
            }
        }

        f[target][max_odd] as i32
    }
}
// end_submission
