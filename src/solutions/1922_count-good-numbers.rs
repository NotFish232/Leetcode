#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    const MOD: i64 = 1_000_000_007;
    
    fn mod_pow(mut base: i64, mut exp: i64) -> i64 {
        let mut res = 1;

        while exp > 0 {
            if exp % 2 == 1 {
                res = (res * base) % Self::MOD;
            }

            base = (base * base) % Self::MOD;
            exp /= 2;
        }

        res
    }

    pub fn count_good_numbers(n: i64) -> i32 {
        ((Self::mod_pow(5, (n + 1) / 2) * Self::mod_pow(4, n / 2)) % Self::MOD) as i32
    }
}
// end_submission
