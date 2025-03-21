use std::{cmp::Ordering, collections::HashMap};

struct DSU {
    parents: Vec<usize>,
    ranks: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> DSU {
        DSU {
            parents: (0..n).collect(),
            ranks: vec![0; n],
        }
    }

    fn find(self: &mut Self, i: usize) -> usize {
        if self.parents[i] != i {
            self.parents[i] = Self::find(self, self.parents[i]);
        }

        self.parents[i]
    }

    fn union(self: &mut Self, a: usize, b: usize) {
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
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut dsu = DSU::new(nums.len());
        let mut h = HashMap::new();

        for (i, &n) in nums.iter().enumerate() {
            if !h.contains_key(&n) {
                if let Some(&low) = h.get(&(n - 1)) {
                    dsu.union(low, i);
                }
                if let Some(&high) = h.get(&(n + 1)) {
                    dsu.union(i, high);
                }

                h.insert(n, i);
            }
        }

        let mut dsu_counts = HashMap::new();

        for i in 0..nums.len() {
            *dsu_counts.entry(dsu.find(i)).or_insert(0) += 1;
        }

        *dsu_counts.values().max().unwrap_or(&0)
    }
}
