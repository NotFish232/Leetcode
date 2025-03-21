use std::{cmp::Ordering, collections::HashSet};

struct DSU {
    parents: Vec<usize>,
    ranks: Vec<i32>,
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
                    self.parents[b_rep] = a_rep;
                    self.ranks[a_rep] += 1;
                }
            }
        }
    }
}

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut dsu = DSU::new(m * n);

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == '1' {
                    if i > 0 && grid[i - 1][j] == '1' {
                        dsu.union((i - 1) * n + j, i * n + j);
                    }
                    if j > 0 && grid[i][j - 1] == '1' {
                        dsu.union(i * n + j - 1, i * n + j);
                    }
                }
            }
        }

        let mut seen_islands = HashSet::new();

        for i in 0..m * n {
            if grid[i / n][i % n] == '1' {
                seen_islands.insert(dsu.find(i));
            }
        }

        seen_islands.len() as i32
    }
}
