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

    fn query(&self, l_idx: usize, r_idx: usize) -> i32 {
        let (mut l_ptr, mut r_ptr) = (self.n + l_idx - 1, self.n + r_idx);

        let mut sum = 0;

        while l_ptr < r_ptr {
            if l_ptr % 2 == 0 {
                sum |= self.tree[l_ptr];
                l_ptr += 1;
            }
            if r_ptr % 2 == 0 {
                sum |= self.tree[r_ptr - 1];
            }

            l_ptr = (l_ptr - 1) / 2;
            r_ptr = (r_ptr - 1) / 2;
        }

        sum
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
