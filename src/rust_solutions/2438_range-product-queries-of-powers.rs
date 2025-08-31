#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    const MOD: i64 = 1_000_000_007;

    fn pow(mut base: i64, mut pow: i32) -> i32 {
        let mut res = 1;

        while pow > 0 {
            if pow % 2 == 1 {
                res = (res * base) % Self::MOD;
            }

            base = (base * base) % Self::MOD;
            pow /= 2;
        }

        res as i32
    }

    pub fn product_queries(mut n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut p_sums = vec![0];

        let mut i = 0;
        while n > 0 {
            if (n & 1) == 1 {
                p_sums.push(p_sums.last().unwrap() + i);
            }
            n >>= 1;

            i += 1;
        }

        let mut ans = Vec::new();

        for q in queries {
            ans.push(Self::pow(
                2,
                p_sums[q[1] as usize + 1] - p_sums[q[0] as usize],
            ))
        }

        ans
    }
}
// end_submission
