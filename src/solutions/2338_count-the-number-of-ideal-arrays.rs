#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    const MOD: i64 = 1_000_000_007;

    pub fn ideal_arrays(n: i32, max_value: i32) -> i32 {
        const MAX_P: usize = 15;

        let mut sieve = vec![0; max_value as usize + 1];
        for i in 2..sieve.len() {
            if sieve[i] == 0 {
                for j in (i..sieve.len()).step_by(i) {
                    sieve[j] = i as i32;
                }
            }
        }

        let mut ps = vec![vec![]; sieve.len()];
        for i in 2..=max_value as usize {
            let mut x = i;
            while x > 1 {
                let p = sieve[x] as usize;

                let mut c = 0;
                while x % p == 0 {
                    x /= p;
                    c += 1;
                }

                ps[i].push(c);
            }
        }

        let mut c = vec![vec![0; MAX_P + 1]; n as usize + MAX_P + 1];
        c[0][0] = 1;
        for i in 1..n as usize + MAX_P + 1 {
            c[i][0] = 1;
            for j in 1..=i.min(MAX_P) {
                c[i][j] = (c[i - 1][j] + c[i - 1][j - 1]) % Self::MOD;
            }
        }

        let mut ans = 0i64;

        for x in 1..=max_value as usize {
            let mut res = 1i64;

            for &p in &ps[x] {
                res *= c[(n + p) as usize - 1][p as usize];
                res %= Self::MOD;
            }

            ans += res;
            ans %= Self::MOD;
        }

        ans as i32
    }
} // end_submission
