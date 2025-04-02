use itertools::Itertools;

impl Solution {
    fn gcd(a: i64, b: i64) -> i64 {
        if b != 0 { Self::gcd(b, a % b) } else { a }
    }

    fn lcm(a: i64, b: i64) -> i64 {
        (a * b) / Self::gcd(a, b)
    }

    fn lcm_n(nums: &[&i64]) -> i64 {
        let mut res = 1;

        for &&num in nums {
            res = Self::lcm(res, num);
        }

        res
    }

    fn count_num(amt: i64, coins: &[i64]) -> i64 {
        let mut count = 0;

        for i in 1..=coins.len() {
            for combo in coins.iter().combinations(i) {
                let sign = if combo.len() % 2 == 1 { 1 } else { -1 };

                let lcm = Self::lcm_n(&combo);

                count += sign * amt / lcm;
            }
        }

        count
    }

    pub fn find_kth_smallest(coins: Vec<i32>, k: i32) -> i64 {
        let coins: Vec<_> = coins.into_iter().map(|c| c as i64).collect();

        let (mut l, mut r) = (0, i64::MAX);

        while l < r {
            let m = l + (r - l) / 2;

            if Self::count_num(m, &coins) >= k as i64 {
                r = m
            } else {
                l = m + 1;
            }
        }

        l
    }
}
