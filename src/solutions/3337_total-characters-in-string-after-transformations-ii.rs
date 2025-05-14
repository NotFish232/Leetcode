#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission

impl Solution {
    const MOD: i64 = 1_000_000_007;

    fn mat_mul(a: &[Vec<i64>], b: &[Vec<i64>]) -> Vec<Vec<i64>> {
        let mut out = vec![vec![0; b[0].len()]; a.len()];

        for i in 0..a.len() {
            for j in 0..b[0].len() {
                for k in 0..a[0].len() {
                    out[i][j] = (out[i][j] + a[i][k] * b[k][j]) % Self::MOD;
                }
            }
        }

        out
    }

    fn mat_exp(base: &[Vec<i64>], mut exp: i32) -> Vec<Vec<i64>> {
        let mut out = vec![vec![0; base[0].len()]; base.len()];
        let mut base: Vec<Vec<_>> = base.to_vec();

        for i in 0..out.len() {
            out[i][i] = 1;
        }

        while exp > 0 {
            if exp % 2 == 1 {
                out = Self::mat_mul(&out, &base);
            }

            base = Self::mat_mul(&base, &base);
            exp /= 2;
        }

        out
    }

    pub fn length_after_transformations(s: String, t: i32, nums: Vec<i32>) -> i32 {
        let mut v = vec![vec![0]; 26];
        for &b in s.as_bytes() {
            v[(b - b'a') as usize][0] += 1;
        }

        let mut num_mat = vec![vec![0; 26]; 26];
        for (i, num) in nums.into_iter().enumerate() {
            let mut v = i;

            for _ in 0..num {
                v = (v + 1) % 26;
                num_mat[v][i] = 1;
            }
        }

        let mut out = Self::mat_exp(&num_mat, t);
        out = Self::mat_mul(&out, &v);

        out.into_iter().fold(0, |acc, v| (acc + v[0]) % Self::MOD) as i32
    }
}
// end_submission
