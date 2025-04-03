#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
use std::{cmp::min, collections::BinaryHeap};

#[allow(dead_code)]
impl Solution {
    const MOD: i64 = 1_000_000_007;

    fn power_mod(mut base: i64, mut exp: i64) -> i64 {
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

    fn get_primes(n: i32) -> Vec<i32> {
        let mut v = vec![true; n as usize + 1];
        let mut primes = Vec::new();

        for i in 2..=n as usize {
            if v[i] {
                primes.push(i as i32);

                let mut j = i * i;
                while j <= n as usize {
                    v[j] = false;
                    j += i;
                }
            }
        }

        primes
    }

    fn prime_score(mut num: i32, primes: &Vec<i32>) -> i32 {
        let mut score = 0;

        for &prime in primes {
            if prime * prime > num {
                break;
            }

            if num % prime != 0 {
                continue;
            }

            score += 1;

            while num % prime == 0 {
                num /= prime;
            }
        }

        if num > 1 {
            score += 1;
        }

        score
    }

    pub fn maximum_score(nums: Vec<i32>, mut k: i32) -> i32 {
        let n = nums.len();

        let max_num = *nums.iter().max().unwrap();

        let primes = Self::get_primes(max_num);

        let prime_scores: Vec<_> = nums
            .iter()
            .map(|&n| Self::prime_score(n, &primes))
            .collect();

        let mut prev_d = vec![-1i32; n];
        let mut next_d = vec![n as i32; n];

        let mut stack = Vec::new();

        for i in 0..n {
            while let Some(&top) = stack.last() {
                if prime_scores[i] <= prime_scores[top] {
                    break;
                }

                next_d[top] = i as i32;
                stack.pop();
            }

            if !stack.is_empty() {
                prev_d[i] = *stack.last().unwrap() as i32;
            }

            stack.push(i);
        }

        let mut num_sub = vec![0; n];
        for i in 0..n {
            num_sub[i] = ((next_d[i] - i as i32) as i64) * ((i as i32 - prev_d[i]) as i64);
        }

        let mut pq = BinaryHeap::new();

        for i in 0..n {
            pq.push((nums[i] as i64, i));
        }

        let mut score = 1;

        while k > 0 {
            let (num, i) = pq.pop().unwrap();

            let ops = min(k as i64, num_sub[i]);

            score = (score * Self::power_mod(num, ops)) % Self::MOD;
            k -= ops as i32;
        }

        score as i32
    }
}
// end_submission
