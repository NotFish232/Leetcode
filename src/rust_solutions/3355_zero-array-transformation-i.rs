#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
struct STree {
    v: Vec<i32>,
    n: usize,
}

impl STree {
    fn new(nodes: Vec<i32>) -> Self {
        let n = nodes.len();
        let mut v = vec![0; 2 * n];

        for i in 0..nodes.len() {
            v[n + i - 1] = nodes[i];
        }
        for i in (0..nodes.len() - 1).rev() {
            v[i] = v[2 * i + 1] + v[2 * i + 2];
        }

        Self { v, n }
    }

    fn update(&mut self, idx: usize, val: i32) {
        let mut p = self.n + idx - 1;

        self.v[p] = val;

        while p > 0 {
            p = (p - 1) / 2;
            self.v[p] = self.v[2 * p + 1] + self.v[2 * p + 2];
        }
    }

    fn get(&self, idx: usize) -> i32 {
        self.v[self.n + idx - 1]
    }

    fn query(&self, idx_1: usize, idx_2: usize) -> i32 {
        let (mut lp, mut rp) = (self.n + idx_1 - 1, self.n + idx_2 - 1);

        let mut res = 0;

       loop {
            if lp % 2 == 0 {
                res += self.v[lp];
                lp += 1;
            }
            if rp % 2 == 1 {
                res += self.v[rp];
                rp -= 1;
            }

            if lp > rp {
                break;
            }

            lp = (lp - 1) / 2;
            rp = (rp - 1) / 2;
        }

        res
    }
}

impl Solution {
    pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
        let mut s_tree = STree::new(vec![0; nums.len()]);

        for q in queries {
            let (idx_1, idx_2) = (q[0] as usize, q[1] as usize);
            s_tree.update(idx_1, s_tree.get(idx_1) + 1);
            if idx_2 + 1 < nums.len() {
                s_tree.update(idx_2 + 1, s_tree.get(idx_2 + 1) - 1);
            }
        }

        let mut p_sum = 0;
        for i in 0..nums.len() {
            p_sum += s_tree.get(i);

            if p_sum < nums[i] {
                return false;
            }
        }

        true
    }
}
// end_submission
