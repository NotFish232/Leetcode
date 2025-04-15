#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
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
            tree[i] = tree[2 * i + 1] + tree[2 * i + 2];
        }

        Self { tree, n }
    }

    fn update(&mut self, idx: usize, val: i32) {
        let mut p = self.n + idx - 1;

        self.tree[p] = val;

        while p > 0 {
            p = (p - 1) / 2;

            self.tree[p] = self.tree[2 * p + 1] + self.tree[2 * p + 2];
        }
    }

    fn query(&mut self, idx_1: usize, idx_2: usize) -> i32 {
        let (mut lp, mut rp) = (self.n + idx_1 - 1, self.n + idx_2);

        let mut res = 0;

        while lp < rp {
            if lp % 2 == 0 {
                res += self.tree[lp];
                lp += 1;
            }
            if rp % 2 == 0 {
                res += self.tree[rp - 1];
            }

            lp = (lp - 1) / 2;
            rp = (rp - 1) / 2;
        }

        res
    }
}

#[allow(dead_code)]
impl Solution {
    pub fn good_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let mut n2_val_to_idx = vec![0; nums1.len()];

        for (i, &n2) in nums2.iter().enumerate() {
            n2_val_to_idx[n2 as usize] = i;
        }

        let positions: Vec<_> = nums1.iter().map(|&n1| n2_val_to_idx[n1 as usize]).collect();

        let mut l_tree = STree::new(vec![0; nums1.len()]);
        let mut r_tree = STree::new(vec![1; nums1.len()]);

        l_tree.update(positions[0], 1);
        r_tree.update(positions[0], 0);

        let mut count = 0;

        for i in 1..nums1.len() - 1 {
            let pos = positions[i];

            l_tree.update(pos, 1);
            r_tree.update(pos, 0);

            let left = l_tree.query(0, pos - 1);
            let right = r_tree.query(pos + 1, nums1.len() - 1);
            count += left as i64 * right as i64;
        }

        count
    }
}
// end_submission
