#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::{
    cmp,
    collections::{BinaryHeap, HashSet},
};

#[derive(PartialEq, Eq, Debug)]
struct Fraction(i32, i32);

impl Ord for Fraction {
    fn cmp(&self, other: &Fraction) -> cmp::Ordering {
        (self.1 * other.0).cmp(&(self.0 * other.1))
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Fraction) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn kth_smallest_prime_fraction(arr: Vec<i32>, mut k: i32) -> Vec<i32> {
        let mut h = BinaryHeap::new();
        h.push((Fraction(arr[0], arr[arr.len() - 1]), 0, arr.len() - 1));

        let mut seen = HashSet::new();
        seen.insert((0, arr.len() - 1));

        while let Some((_, l, r)) = h.pop() {
            if !seen.contains(&(l + 1, r)) {
                h.push((Fraction(arr[l + 1], arr[r]), l + 1, r));
                seen.insert((l + 1, r));
            }
            if !seen.contains(&(l, r - 1)) {
                h.push((Fraction(arr[l], arr[r - 1]), l, r - 1));
                seen.insert((l, r - 1));
            }

            k -= 1;

            if k == 0 {
                return vec![arr[l], arr[r]];
            }
        }

        unreachable!()
    }
}
// end_submission
