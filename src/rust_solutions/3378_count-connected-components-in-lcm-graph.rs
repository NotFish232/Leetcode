#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use itertools::Itertools;
use std::cmp::Ordering;

struct Dsu {
    parents: Vec<usize>,
    ranks: Vec<usize>,
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
            self.parents[i] = Self::find(self, self.parents[i]);
        }

        self.parents[i]
    }

    fn union(&mut self, a: usize, b: usize) {
        let a_rep = Self::find(self, a);
        let b_rep = Self::find(self, b);

        if a_rep != b_rep {
            match self.ranks[a_rep].cmp(&self.ranks[b_rep]) {
                Ordering::Less => self.parents[a_rep] = b_rep,
                Ordering::Greater => self.parents[b_rep] = a_rep,
                Ordering::Equal => {
                    self.parents[a_rep] = b_rep;
                    self.ranks[b_rep] += 1;
                }
            }
        }
    }
}

impl Solution {
    pub fn count_components(nums: Vec<i32>, threshold: i32) -> i32 {
        let mut dsu = Dsu::new(threshold as usize + 1);

        for &x in &nums {
            for y in (2 * x..=threshold).step_by(x as usize) {
                dsu.union(x as usize, y as usize);
            }
        }

        nums.iter()
            .filter_map(|&x| {
                if x <= threshold {
                    Some(dsu.find(x as usize))
                } else {
                    None
                }
            })
            .unique()
            .count() as i32
            + nums.iter().filter(|&&x| x > threshold).count() as i32
    }
}
// end_submission
