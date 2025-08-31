#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::{
    cmp::Ordering,
    collections::{hash_map::Entry, HashMap},
};

struct Dsu {
    parents: Vec<usize>,
    ranks: Vec<i32>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parents: (0..n).collect(),
            ranks: vec![0; n],
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parents[i] != i {
            self.parents[i] = self.find(self.parents[i]);
        }

        self.parents[i]
    }

    fn union(&mut self, a: usize, b: usize) {
        let a_rep = self.find(a);
        let b_rep = self.find(b);

        if a_rep != b_rep {
            match self.ranks[a_rep].cmp(&self.ranks[b_rep]) {
                Ordering::Less => self.parents[a_rep] = b_rep,
                Ordering::Greater => self.parents[b_rep] = a_rep,
                Ordering::Equal => {
                    self.parents[b_rep] = a_rep;
                    self.ranks[a_rep] += 1;
                }
            }
        }
    }
}

impl Solution {
    fn find_prime_factors(mut x: i32) -> Vec<i32> {
        let mut prime_factors = Vec::new();

        if x % 2 == 0 {
            prime_factors.push(2);
            while x % 2 == 0 {
                x /= 2;
            }
        }

        let mut i = 3;
        while i * i <= x {
            if x % i == 0 {
                prime_factors.push(i);
                while x % i == 0 {
                    x /= i;
                }
            }
            i += 2;
        }

        if x > 1 {
            prime_factors.push(x);
        }

        prime_factors
    }

    pub fn largest_component_size(nums: Vec<i32>) -> i32 {
        let mut dsu = Dsu::new(nums.len());

        let mut factor_to_i = HashMap::new();

        for i in 0..nums.len() {
            let prime_factors = Self::find_prime_factors(nums[i]);

            for f in prime_factors {
                if let Entry::Vacant(e) = factor_to_i.entry(f) {
                    e.insert(i);
                } else {
                    dsu.union(i, factor_to_i[&f]);
                }
            }
        }

        let mut dsu_counts = HashMap::new();

        for i in 0..nums.len() {
            *dsu_counts.entry(dsu.find(i)).or_insert(0) += 1;
        }

        *dsu_counts.values().max().unwrap_or(&0)
    }
}
// end_submission
