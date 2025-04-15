struct SegmentTree {
    tree: Vec<i32>,
    n: usize,
}

impl SegmentTree {
    fn new(v: Vec<i32>) -> Self {
        let n = v.len();
        let mut tree = vec![0; 2 * n];

        for i in 0..n {
            tree[n + i - 1] = v[i];
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
        let (mut l_ptr, mut r_ptr) = (self.n + idx_1 - 1, self.n + idx_2);

        let mut res = 0;

        while l_ptr < r_ptr {
            if l_ptr % 2 == 0 {
                res += self.tree[l_ptr];
                l_ptr += 1;
            }
            if r_ptr % 2 == 0 {
                res += self.tree[r_ptr - 1];
            }

            l_ptr = (l_ptr - 1) / 2;
            r_ptr = (r_ptr - 1) / 2;
        }

        res
    }
}
