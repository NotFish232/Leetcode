#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
struct Dsu {
    parents: Vec<usize>,
    size: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parents: (0..n).collect(),
            size: vec![1; n],
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
            if self.size[a_rep] < self.size[b_rep] {
                self.parents[a_rep] = b_rep;
                self.size[b_rep] += self.size[a_rep];
            } else {
                self.parents[b_rep] = a_rep;
                self.size[a_rep] += self.size[b_rep];
            }
        }
    }
}

#[allow(dead_code)]
impl Solution {
    pub fn max_points(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let (n, m) = (grid.len(), grid[0].len());

        let mut grid_squares = Vec::new();
        for i in 0..n {
            for j in 0..m {
                grid_squares.push((grid[i][j], i, j));
            }
        }
        grid_squares.sort();

        let mut q_idxs: Vec<_> = (0..queries.len()).collect();
        q_idxs.sort_by_key(|&i| queries[i]);

        let mut v = vec![0; queries.len()];

        let mut dsu = Dsu::new(n * m);

        let mut s_idx = 0;

        for idx in q_idxs {
            let q = queries[idx];

            while s_idx < grid_squares.len() && grid_squares[s_idx].0 < q {
                let (_, r, c) = grid_squares[s_idx];

                for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let new_r = r as i32 + dr;
                    let new_c = c as i32 + dc;

                    if 0 <= new_r
                        && new_r < n as i32
                        && 0 <= new_c
                        && new_c < m as i32
                        && grid[new_r as usize][new_c as usize] < q
                    {
                        dsu.union(r * m + c, new_r as usize * m + new_c as usize);
                    }
                }

                s_idx += 1;
            }

            v[idx] = if grid[0][0] < q {
                let d_idx = dsu.find(0);
                dsu.size[d_idx] as i32
            } else {
                0
            };
        }

        v
    }
}
// end_submission
