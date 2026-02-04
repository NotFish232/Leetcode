#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
struct STree {
    tree: Vec<i32>,
    n: usize,
}

impl STree {
    fn new(nodes: Vec<i32>) -> Self {
        let n = nodes.len();

        let mut tree = vec![0; 2 * n];

        for i in 0..n {
            tree[n + i - 1] = nodes[i];
        }

        for i in (0..n - 1).rev() {
            tree[i] = tree[2 * i + 1] | tree[2 * i + 2];
        }

        Self { tree, n }
    }

    fn update(&mut self, idx: usize, val: i32) {
        let mut p = self.n + idx - 1;

        self.tree[p] = val;

        while p > 0 {
            p = (p - 1) / 2;

            self.tree[p] = self.tree[2 * p + 1] | self.tree[2 * p + 2];
        }
    }

    fn query(&self, idx_1: usize, idx_2: usize) -> i32 {
        let (mut lp, mut rp) = (self.n + idx_1 - 1, self.n + idx_2 - 1);

        let mut res = 0;

        loop {
            if lp % 2 == 0 {
                res |= self.tree[lp];
                lp += 1;
            }
            if rp % 2 == 1 {
                res |= self.tree[rp];
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
    pub fn smallest_subarrays(nums: Vec<i32>) -> Vec<i32> {
        let mut max_ors = vec![0; nums.len()];
        let mut cur_or = 0;
        for (i, &x) in nums.iter().enumerate().rev() {
            cur_or |= x;
            max_ors[i] = cur_or;
        }

        let mut res = Vec::new();

        let mut s_tree = STree::new(vec![0; nums.len()]);

        let mut j = 0;
        for (i, &t_or) in max_ors.iter().enumerate() {
            while i == j || s_tree.query(0, nums.len() - 1) != t_or {
                s_tree.update(j, nums[j]);
                j += 1;
            }

            res.push((j - i) as i32);

            s_tree.update(i, 0);
        }

        res
    }
}
// end_submission
